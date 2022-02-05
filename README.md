# Dead Router

A server to continously poll nearly always-on sites to verify that your internet connectivity stays
up!

If one or more of the servers stops responding to pings, the server tracks that start time, and
once it comes back online, sends a discord notification to your configured webhook, with timing
information about the downtime, and which sites didn't respond.

## Usage

Check `.env.example`, and make a corresponding `.env` file with filled in values. Then run
`docker-compose up -d --build` to run the image and start the server.
