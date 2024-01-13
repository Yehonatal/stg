slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    
    let ui_divide_income = ui_handle.clone();
    ui.on_divide_income(move |string| {
        let ui = ui_divide_income.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let _tax: f64 = num * TAXPER;
        let _owner: f64 = num * OWNERPER;
        let _profit: f64 = num * PROFITPER;
        let _opx: f64 = num * OPEXPER;

        let results = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", _tax, _owner, _profit, _opx);

        ui.set_result(results.into())
    });


    ui.run()
}
