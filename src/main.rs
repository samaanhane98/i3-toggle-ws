use std::rc::Rc;
use tokio_i3ipc::{reply::Workspace, I3};

mod error;
use error::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut i3 = I3::connect().await?;
    let workspaces = i3.get_workspaces().await?;

    let active_workspace = workspaces
        .clone()
        .into_iter()
        .find(|workspace| workspace.focused)
        .ok_or("I3 error, no focussed workspace")?;

    let active_output = active_workspace.output;

    let output_workspaces: Rc<[Workspace]> = workspaces
        .clone()
        .into_iter()
        .filter(|workspace| workspace.output == active_output)
        .collect();

    let next_workspace = output_workspaces
        .iter()
        .cycle()
        .skip_while(|workspace| workspace.name != active_workspace.name)
        .skip(1)
        .next()
        .ok_or("Next workspace could not be determined")?;

    let msg_body = format!("workspace \"{}\"", next_workspace.name);
    i3.send_msg_body(tokio_i3ipc::msg::Msg::RunCommand, msg_body)
        .await?;

    Ok(())
}
