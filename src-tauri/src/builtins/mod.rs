use crate::action::function_call::FunctionCall;

// pub const BUILTINS_VARIABLES: [&str; 1] = ["PI"]; // TODO
pub const BUILTINS_FUNCTIONS_LIST: [&str; 4] = ["print", "exit", "sleep", "math"];
// TODO un moyen plus elegant de faire ca ? (qui s'actualise automatiquement avec une macro ?)
pub const JSON_BUILTINS_FUNCTIONS: &'static str = r#"
[
    {
        "name": "print",
        "args": [
            {
                "name": "text",
                "type": "text"
            }
        ],
        "is_variadic": true
    },
    {
        "name": "exit",
        "args": [],
        "is_variadic": false
    },
]
"#;

#[tauri::command]
pub fn get_builtins_functions_list() -> Vec<&'static str> {
    BUILTINS_FUNCTIONS_LIST.into()
}

#[tauri::command]
fn print(args: Vec<String>) {
    args.iter().for_each(|arg| {
        print!("{} ", arg);
    });
    println!();
}

pub fn execute_native_function(
    function: &FunctionCall,
    //functions: &mut FunctionsMap,
    //variables: &mut VariablesMap,
) -> Result<Option<()>, ()> {
    match &function.name as &str {
        "print" | "println" => {
            for arg in function.args.iter() {
                println!("salut {:?}" , arg);
                //let value = arg.get_value(functions, variables);
                //print!("{}", value);
            }
            if function.name == "println" {
                println!();
            }
            return Ok(None);
        }
        "less_than"
        | "equal_than"
        | "greater_than"
        | "less_or_equal_than"
        | "greater_or_equal_than"
        | "not_equal_than" => {
            if function.args.len() != 2 {
                todo!("{} function takes 2 arguments", function.name);
            }
            /*
            let left = function.args[0].get_value(functions, variables);
            let right = function.args[1].get_value(functions, variables);
            return Ok(Some(Variable::from(comparaison(
                left,
                &function.name,
                right,
            ))));
             */
        }
        "add" => {
            if function.args.len() != 2 {
                todo!(
                    "add function takes 2 arguments (and not {})",
                    function.args.len()
                );
            }
            /*
            let left = function.args[0].get_value(functions, variables);
            let right = function.args[1].get_value(functions, variables);
            return Ok(Some(Variable::from(add(&left, &right))));

             */
        }
        _ => {
            panic!("Unknown native function: {}", function.name)
        }
    }
    Err(())
}
