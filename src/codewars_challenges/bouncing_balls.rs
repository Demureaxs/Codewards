pub fn run_bouncing_balls() {
    bouncing_ball(3.0, 0.66, 1.5);
    // bouncing_ball(40.0, 0.4, 10.0);
    // bouncing_ball(30.0, 0.66, 1.5);
    // bouncing_ball(10.0, 0.6, 10.0);
    // bouncing_ball(2.0, 0.5, 1.0);
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h < 1.0 || bounce >= 1.0 || bounce <= 0.0 || window >= h {
        return -1;
    }
    let mut bounce_height = h;
    let mut bounces = 0;

    while bounce_height > window {
        bounces += 1;
        bounce_height = bounce_height * bounce;
        if bounce_height > window {
            bounces += 1;
        }
    }
    bounces
}