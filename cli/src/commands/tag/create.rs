// Copyright 2025 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cli_util::CommandHelper;
use crate::cli_util::RevisionArg;
use crate::command_error::CommandError;
use crate::complete;
use crate::ui::Ui;
use clap_complete::ArgValueCompleter;

#[derive(clap::Args, Clone, Debug)]
pub struct TagCreateArgs {
    /// Tag to create
    #[arg(required = true)]
    pub tag: String,

    /// The tag's target revision
    #[arg(
        long, short,
        visible_alias = "to",
        value_name = "REVSET",
        add = ArgValueCompleter::new(complete::revset_expression_all),
    )]
    revision: Option<RevisionArg>,
}

pub fn cmd_tag_create(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &TagCreateArgs,
) -> Result<(), CommandError> {
    // let workspace_command = command.workspace_helper(ui)?;
    // println!("UI: {}", ui);
    // println!("Command: {}", command);
    // println!("Args: {}", args);

    writeln!(ui.hint_default(), "Tag create")?;

    Ok(())
}
