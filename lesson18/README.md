In this assignment, you will add monitoring capabilities to the server part of your chat application using Prometheus. Monitoring is a crucial aspect of maintaining and understanding the health and performance of applications, especially in production environments.
Description:

    Integrate Prometheus:
        Add Prometheus to your chat application's server.
        Ensure that Prometheus is set up correctly to gather metrics from your server.

    Metrics Implementation:
        Implement at least one metric using Prometheus. At a minimum, add a counter to track the number of messages sent through your server.
        Optionally, consider adding a gauge to monitor the number of active connections to your server. This can provide insights into user engagement and server load.

    Metrics Endpoint:
        Set up an endpoint within your server application to expose these metrics to Prometheus. This typically involves creating a /metrics endpoint.
        Ensure that the endpoint correctly exposes the metrics in a format that Prometheus can scrape.

Typically, this means using the TextEncoder: https://docs.rs/prometheus/0.13.3/prometheus/struct.TextEncoder.html

You can refer to the Hyper example: https://github.com/tikv/rust-prometheus/blob/master/examples/example_hyper.rs

    Documentation and Testing:
        Document the new metrics feature in your README.md, including how to access the metrics endpoint and interpret the exposed data.
        Test to make sure that the metrics are accurately recorded and exposed. Verify that Prometheus can successfully scrape these metrics from your server.
