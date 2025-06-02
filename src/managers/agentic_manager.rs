use eyre::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;

use crate::agentic::agent::Agent;
use crate::agentic::history::Message;
use crate::agentic::response::AgenticResponse;
use crate::agentic::tools::calculator::CalculatorTool;
use crate::agentic::tools::file_manager::FileManagerTool;
use crate::agentic::tools::tool::Tool;
use crate::integrations::openwebui::openwebui_service::OpenWebUIService;
use crate::utils::config::Config;
use crate::utils::text_extraction::get_input_or_stdin;

pub struct AgenticManager {
    config: Config,
    agents: Vec<Agent>,
    teams: HashMap<String, String>,
    teams_dir: String,
    manager_role: String,

    owui_client: OpenWebUIService,
}

impl AgenticManager {
    pub fn new(teams_directory: Option<String>) -> Result<AgenticManager> {
        let config_struct = Config::load_configuration_struct();

        let teams_directory = teams_directory
            .or_else(|| {
                if !config_struct.team_directory.is_empty() {
                    Some(config_struct.team_directory.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| String::from("./teams"));

        tracing::debug!("teams dir: {}", teams_directory);
        tracing::debug!("owui base url: {}", config_struct.owui_base_url);
        tracing::debug!(
            "owui auth token len: {}",
            config_struct.owui_auth_token.len()
        );

        let mut app_manager = AgenticManager {
            config: config_struct.clone(),
            agents: Vec::new(),
            teams: HashMap::new(),
            teams_dir: teams_directory,
            manager_role: "".to_string(),
            owui_client: OpenWebUIService::new(
                config_struct.owui_base_url.as_str(),
                config_struct.owui_auth_token.as_str(),
            ),
        };

        app_manager.load_teams()?;

        Ok(app_manager)
    }

    fn load_teams(&mut self) -> Result<()> {
        let entries = fs::read_dir(self.teams_dir.clone())?;

        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if let Ok(path) = entry.path().into_os_string().into_string() {
                                self.teams.insert(file_name, path);
                            }
                        }
                    }
                }
            }
        }

        tracing::debug!("loaded {} teams", self.teams.len());

        Ok(())
    }

    fn load_agents_from_dir(&mut self, dir: &str) -> Result<()> {
        // Read the directory entries
        let entries: ReadDir = fs::read_dir(dir)?;

        // Process each entry in the directory
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Skip if not a file or not a JSON file
            if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            // Read and parse the JSON file
            let json_content = fs::read_to_string(&path)?;
            match Agent::from_json(&json_content) {
                Ok(agent) => {
                    if agent.is_manager {
                        self.manager_role = agent.role.clone();
                    }

                    tracing::info!(
                        "Loaded agent from {}: {}: {}",
                        path.display().to_string(),
                        agent.role,
                        agent.is_manager
                    );
                    self.agents.push(agent);
                }
                Err(e) => tracing::error!("Failed to load agent from {:?}: {}", path, e),
            }
        }

        Ok(())
    }

    pub fn process_task(
        &mut self,
        model_name: Option<String>,
        team: &str,
        task: Option<String>,
    ) -> Result<()> {
        // Get the input for the query
        let input = get_input_or_stdin(task.to_owned());

        if input.is_empty() {
            return Err(eyre::eyre!("input is empty"));
        }

        let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(FileManagerTool)];

        match self.teams.clone().get(team) {
            Some(team_dir) => self.load_agents_from_dir(team_dir)?,
            None => return Err(eyre::eyre!("team '{}' not found", team)),
        }

        let agent_details = self
            .agents
            .iter()
            .map(|agent| format!("Role: {} - Goal: {}", agent.role, agent.goal))
            .collect::<Vec<_>>()
            .join("\n");

        let mut current_agent = self.manager_role.clone();

        // Iterate over the agents and run the current agent
        let mut agent = self
            .agents
            .iter_mut()
            .find(|a| a.role.to_lowercase() == current_agent.to_lowercase())
            .ok_or_else(|| eyre::eyre!("Agent with role '{}' not found", current_agent))?;

        agent.context.add(Message {
            role: "user".to_string(),
            content: agent.construct_manager_prompt(input.as_str(), agent_details.as_str())?,
        });

        let model = model_name.unwrap_or_else(|| self.config.model_name.clone());
        let mut previous_report = String::new();

        // Iterate 100 times trying to solve the task
        for _ in 0..100 {
            tracing::info!(
                "Submitting - LLM Processing completion: {}",
                agent.context.print_history().len()
            );

            let completion =
                self.owui_client
                    .completion(&model, format!("# Relevant Data:\n{}\n", previous_report).as_str(), agent.context.print_history().as_str())?;

            // Check if our completion failed if it did we retry
            let Ok(rbop_completion) = AgenticResponse::from_completion(completion.as_str()) else {
                agent.context.add(Message {
                    role: "user".to_string(),
                    content: agent.construct_invalid_format_prompt()?,
                });
                continue;
            };

            tracing::info!("--------------------------");
            rbop_completion.print_result(current_agent.as_str());
            tracing::info!("--------------------------");

            agent.context.add(Message {
                role: "assistant".to_string(),
                content: completion,
            });

            match rbop_completion.action.as_str() {
                "delegate" => {
                    tracing::info!("Delegating from {} to {}", agent.role, rbop_completion.data);

                    agent = self
                        .agents
                        .iter_mut()
                        .find(|a| a.role.to_lowercase() == rbop_completion.data.to_lowercase())
                        .ok_or_else(|| {
                            eyre::eyre!("Agent with role '{}' not found", rbop_completion.data)
                        })?;

                    current_agent = rbop_completion.data.clone();

                    agent.context.add(Message {
                        role: "user".to_string(),
                        content: agent
                            .construct_team_prompt(rbop_completion.data2.as_str(), &tools)?,
                    });
                }
                "tool" => {
                    let tool = tools
                        .iter()
                        .find(|t| t.name() == rbop_completion.data.as_str())
                        .ok_or_else(|| eyre::eyre!("Tool '{}' not found", rbop_completion.data))?;

                    let tool_response = tool.run(rbop_completion.data2.as_str())?;

                    tracing::info!(
                        "Tool Run: {} Tool Response:{}",
                        rbop_completion.data2.as_str(),
                        tool_response
                    );

                    agent.context.add(Message {
                        role: "user".to_string(),
                        content: format!("Result: {}", tool_response),
                    });
                }
                "report" => {
                    agent = self
                        .agents
                        .iter_mut()
                        .find(|a| a.role.to_lowercase() == self.manager_role.to_lowercase())
                        .ok_or_else(|| {
                            eyre::eyre!("Agent with role '{}' not found", self.manager_role)
                        })?;

                    agent.context.add(Message {
                        role: "user".to_string(),
                        content: format!("Result: {}", rbop_completion.data),
                    });

                    previous_report = format!("{}\n{}",previous_report,rbop_completion.data.clone());

                    current_agent = self.manager_role.clone();
                }
                "answer" => {
                    tracing::info!("Task Completed Results: {}\n", rbop_completion.data);
                    break;
                }
                _ => {
                    tracing::error!("Invalid action selected: {}", rbop_completion.action);
                    tracing::warn!("Retrying");

                    agent.context.remove_latest();
                    continue;
                }
            }
        }

        Ok(())
    }
}
