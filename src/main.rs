use swayipc::{Connection, EventType};
use swayipc::{Event, Node, NodeType, WindowChange};

use clap::Parser;

fn switch_tiling(conn: &mut Connection, workspaces: &[i32], master_ratio: f32) -> Result<(), String> {
    // Check if focused workspace is in "allowed list".
    // If `workspaces` is empty, skip allow all workspaces.
    if !workspaces.is_empty() {
        for workspace in conn
            .get_workspaces()
            .map_err(|_| "get_workspaces() failed")?
        {
            if workspace.focused {
                if workspaces.contains(&workspace.num) {
                    break;
                } else {
                    return Ok(());
                }
            }
        }
    }

    // Get the tree and find the focused node
    let tree = conn.get_tree().map_err(|_| "get_tree() failed")?;
    // TODO improve the logic
    let _focused_node = tree
        .find_focused_as_ref(|n| n.focused)
        .ok_or("Could not find the focused node")?;

    let parent = tree
        .find_focused_as_ref(|n| n.nodes.iter().any(|n| n.focused))
        .ok_or("No parent")?;

    // Check if the parent is a workspace
    if parent.node_type != NodeType::Workspace {
        return Ok(());
    }

    // Get the nodes (windows) under the workspace
    let nodes: Vec<&Node> = parent.nodes.iter().collect();
    let num_windows = nodes.len();

    if num_windows == 0 {
        return Ok(());
    }

    // Define the master and stack areas
    let master_count = 1; // You can adjust this if you want more master windows
    // TODO improve the logic
    let _stack_count = num_windows - master_count;

    // Determine the layout for the master area and stack area
    for (i, node) in nodes.iter().enumerate() {
        let layout_cmd = if i < master_count {
            "splith"
        } else {
            "splitv"
        };

        conn.run_command(format!("[con_id={}] {}", node.id, layout_cmd)).unwrap();
    }

    // Adjust the size of the master area
    if let Some(master_node) = nodes.get(0) {
        // TODO improve the logic
        let _resize_cmd = format!(
            "[con_id={}] resize set width {}ppt",
            master_node.id,
            (master_ratio * 100.0) as i32
        );
        // conn.run_command(resize_cmd).unwrap();
    }

    Ok(())
}

#[derive(Parser)]
#[clap(version, author, about)]
struct Cli {
    /// Activate dynamic tiling only on this workspace. More than one workspace may be specified.
    #[clap(long, short = 'w')]
    workspace: Vec<i32>,
    /// Set the master area ratio (default is 0.6).
    #[clap(long, short = 'm', default_value = "0.5")]
    master_ratio: f32,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let mut conn = Connection::new().unwrap();
    for event in Connection::new()
        .unwrap()
        .subscribe(&[EventType::Window])
        .unwrap()
    {
        match event.unwrap() {
            Event::Window(e) => {
                if let WindowChange::Focus = e.change {
                    // We can not use the e.container because the data is stale.
                    // If we compare that node data with the node given from get_tree() after we
                    // delete a node we find that the e.container.rect.height and e.container.rect.width are stale,
                    // and therefore we make the wrong decision on which layout our next window should be.
                    // Refer to https://github.com/swaywm/sway/issues/5873
                    if let Err(err) = switch_tiling(&mut conn, &args.workspace, args.master_ratio) {
                        eprintln!("err: {}", err);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
