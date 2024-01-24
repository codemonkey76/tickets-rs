#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// region:   --- Login
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"password": "welcome"
		}),
	);

	req_login.await?.print().await?;
	// endregion: --- Login

	// region:    --- Create Tasks
	let mut task_ids: Vec<i64> = Vec::new();
	for i in 0..=4 {
		let req_create_task = hc.do_post(
			"/api/rpc",
			json!({
				"id": 1,
				"method": "create_task",
				"params": {
					"data": {
						"title": format!("Task AAA {}", i)
					}
				}
			}),
		);
		let result = req_create_task.await?;

		task_ids.push(result.json_value::<i64>("/result/id")?);
	}
	// endregion: --- Create Tasks

	// region:    --- Update Task
	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": task_ids[0],
				"data": {
					"title": "Task BBB"
				}
			}
		}),
	);
	req_update_task.await?.print().await?;
	// endregion: --- Update Task

	// region:    --- Delete Task
	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": task_ids[1]
			}
		}),
	);
	req_delete_task.await?.print().await?;
	// endregion: --- Delete Task

	// region:    --- List Tasks with filters
	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks",
			"params": {
				"filters": [
					{"title": {"$endsWith": "BBB"}},
					{"id": {"$in": [task_ids[2], task_ids[3]]}}
				],
				"list_options": {"order_bys": "!id"}
			}
		}),
	);

	req_list_tasks.await?.print().await?;
	// endregion: --- List Tasks

	// region:    --- Cleanup
	for id in task_ids {
		// region:    --- Delete Task
		let req_delete_task = hc.do_post(
			"/api/rpc",
			json!({
				"id": 1,
				"method": "delete_task",
				"params": {
					"id": id
				}
			}),
		);
		req_delete_task.await?;
		// endregion: --- Delete Task
	}
	// endregion: --- Cleanup

	// region:    --- Logoff
	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_logoff.await?;
	// endregion: --- Logoff

	Ok(())
}
