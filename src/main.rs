use safe_drive::{
    context::Context, error::DynError, logger::Logger,pr_info
};
use safe_drive::msg::common_interfaces::{
    geometry_msgs, sensor_msgs
};
use std::time::Duration;


fn main()-> Result<(), DynError>{
    let ctx = Context::new()?;

    let node = ctx.create_node("joy_rust", None, Default::default())?;

    let sub_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("/joy",  None)?;
    let pub_joy = node.create_publisher::<geometry_msgs::msg::Twist>("cmd_vel_test", None)?;

    let logger = Logger::new("joy_rust");

    let mut selector = ctx.create_selector()?;

    let mut send_data = geometry_msgs::msg::Twist::new().unwrap();


    selector.add_subscriber(
        sub_joy,
        Box::new(move |msg| {    
            let mut _get_msg = msg.axes.as_slice();
            send_data.linear.x = _get_msg[0] as f64;
            send_data.linear.y = _get_msg[1] as f64;
            send_data.angular.z = _get_msg[3] as f64;
        }),
    );

    loop {
        selector.wait()?;
        pub_joy.send(&send_data)?;

        pr_info!(logger, "send: recieve Joy and send Twist"); 

        std::thread::sleep(Duration::from_secs(1));
    }

}
