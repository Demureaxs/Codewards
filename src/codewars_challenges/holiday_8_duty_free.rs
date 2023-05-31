pub fn run_duty_free() {
    duty_free(12, 50, 1000);
    duty_free(17, 10, 500);
}

fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    let discount_percent = discount as f32 / 100.0;
    let discount_per_bottle = price as f32 * discount_percent;
    let bottles_afforded = (holiday_cost as f32 / discount_per_bottle).floor() as i32;
    return bottles_afforded;
}