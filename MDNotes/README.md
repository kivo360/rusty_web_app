# Goals and How To Read

The goal here is to store all of the notes involving the HTTP calls, websockets, actors, and room logic for the chat. What will happen is a transfer of the concepts to a mini-design document based on the conept/example.

Ultimately, the combination of parts will lead to an end result.

* HTTP Calls
* Websocket Communications
* Room Protocol - The chat protocol will be different from websocket communications. This will be a pubsub pattern that will communicate to the client through websockets. It will communicate the events a user is subscribed to as they come in and get stored on the database.


### High Level TO-DO List

Before going into extreme detail, we should note these are the broad tasks we're to breakdown into the other parts of the application:

1. Label the key service commands
    1. This is synonymous with actually determining how the user will interact with the application.
1. Create empty functions for the key service commands.
    1. Create the functions that return raw json information.
    1. This will be the foundation for sending and receiving proper information.
1. Connection and simple variable extraction
    1. Here we'd only extract the connection we intend to use throughout the application. This connection is to the database.
1. Manage a lazy static `DBCommands` class. You can access this class by reference (`&DBCommands`) between all of the http commands and reduce memory usage as a response. 
    1. As long as the object itself doesn't have any immutable parts, we can easily access the common pools of information.
1. Register User By Id
    1. ID should be from the Twitch API.
    1. We'd be testing if we can add an ID through a post HTTP call.
    1. Create user if not exist. We should already have this done and unit tested.
    1. Print existing user's information if we've already added it by a given ID. This should work as long as we have the `Debug` trait on the User object.
1. Cookie based sessions management.
    1. We'd test the beginning parts of this on the last task.
    1. We'd see if we can set a user, then see if the user exists at the beginning of the call if there's also a cookie sent in.
        1. On te client a cookie will be forced in if we detect it locally.
1. Login/Logout
    1. Extension of last two parts. Removes session from the local server.
1. Update favorite streamer by cookie.
1. Get all sample streamer information from a HTTP command. We'd identifiy the favorite streamer by HTTP command.
1. Ensure the sessions are added into all of the rest of the application.
1. A simple Pub Sub crate:
    1. Create a pub sub library using Rust.
    1. The pub-sub library will include a simple pubsub pattern, and a strategy pattern so I can include any type of storage adaptively.
    1. Done here to isolate development and learn how to properly develop.
1. Create a simple background worker with actix:
    1. Send commands to actix workers using http request.
        1. Print statement at beginning of the core endpoints.
        2. We will also send a message to the main actors using a broadcasting method.
    1. One of the actors will have a forever loop.
1. Attach main websocket to HTTP interface.
1. Ser a background process.