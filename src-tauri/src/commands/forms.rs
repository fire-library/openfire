use crate::domain::{
    method::{
        calculation::CalculationComponent,
        form::FieldTrait,
        parameter::{ArcParameter, Parameter, ParametersTrait},
        validation::ParameterError,
    },
    tab::{TabState, Tabs, WrappedTabState},
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Emitter, State};
use tauri_specta::Event;

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq, Event)]
pub struct ValidationErrorEvent {
    field_id: String,
    error: String,
}

#[tauri::command]
#[specta::specta]
pub async fn calculate_form<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::Method(method) => {
            let validation = method.form.validate();

            if let Ok(_) = validation {
                emit_response(validation, &app_handle)?;
                method.evaluate()?;
                app_handle.emit("tabs_updated", ()).unwrap();
            } else {
                emit_response(validation, &app_handle)?;
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update_field<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
    value: Option<String>,
) -> Result<(), ParameterError> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::Method(method) => {
            let field = method.form.get_field(id);
            method.calc_sheet.as_ref().write().unwrap().stale = true;
            app_handle.emit("tabs_updated", ()).unwrap();
            field.update(value)?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

fn emit_response<R: tauri::Runtime>(
    errors: Result<(), Vec<ParameterError>>,
    app_handle: &AppHandle<R>,
) -> Result<(), String> {
    if let Err(validation_errors) = errors {
        let errors: Vec<ValidationErrorEvent> = validation_errors
            .iter()
            .map(|error| ValidationErrorEvent {
                field_id: error.parameter_id(),
                error: error.message(),
            })
            .collect();

        app_handle
            .emit("validation-error", errors)
            .map_err(|_| "Failed to emit validation error".to_string())?;

        return Err("Validation failed".to_string());
    } else {
        app_handle
            .emit("validation-ok", ())
            .map_err(|_| "Failed to emit validation ok".to_string())?;
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_equation_with_symbols(
    all_tabs_state: State<'_, WrappedTabState>,
    parameter: Parameter,
) -> Result<Vec<Vec<CalculationComponent>>, String> {
    let tabs = all_tabs_state.inner().lock().await;

    let tab = tabs.get_current()?;
    let state = &tab.write().unwrap().state;

    let parameters = match state {
        TabState::Method(form) => &form.parameters,
        _ => return Err("Cannot get equation".to_string()),
    };
    let param = parameters.get_parameter(&parameter.id);
    let param = param.read().unwrap();

    match &param.expression {
        Some(expr) => Ok(expr.generate_with_symbols()),
        None => Err("No expression found".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_equation_with_numbers(
    all_tabs_state: State<'_, WrappedTabState>,
    parameter: Parameter,
) -> Result<Vec<Vec<CalculationComponent>>, String> {
    let tabs = all_tabs_state.inner().lock().await;

    let tab = tabs.get_current()?;
    let state = &tab.write().unwrap().state;

    let parameters = match state {
        TabState::Method(form) => &form.parameters,
        _ => return Err("Cannot get equation".to_string()),
    };
    let param = parameters.get_parameter(&parameter.id);
    let param = param.read().unwrap();

    match &param.expression {
        Some(expr) => Ok(expr.generate_with_values()),
        None => Err("No expression found".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_equation_inputs_symbols(
    all_tabs_state: State<'_, WrappedTabState>,
    step_id: i32,
) -> Result<Vec<ArcParameter>, String> {
    let tabs = all_tabs_state.inner().lock().await;

    let tab = tabs.get_current()?;
    let state = &tab.write().unwrap().state;

    let deps = match state {
        TabState::Method(form) => {
            let step = &form.calc_sheet.read().unwrap().steps[step_id as usize];
            step.get_dependencies()
        }
        _ => return Err("Cannot get equation".to_string()),
    };

    Ok(deps)
}

#[tauri::command]
#[specta::specta]
pub async fn get_equation_inputs(
    all_tabs_state: State<'_, WrappedTabState>,
    step_id: i32,
) -> Result<Vec<ArcParameter>, String> {
    let tabs = all_tabs_state.inner().lock().await;

    let tab = tabs.get_current()?;
    let state = &tab.write().unwrap().state;

    let step = match state {
        TabState::Method(form) => &form.calc_sheet.read().unwrap().steps[step_id as usize],
        _ => return Err("Cannot get equation".to_string()),
    };

    let deps = step.get_input();

    Ok(deps)
}
