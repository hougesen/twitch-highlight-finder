# twitch-highlight-finder

> [!NOTE]
> This project was part of [my undergraduate thesis](https://mhouge.dk/blog/undergraduate-thesis).

The goal of this project was to explore the feasibility of automating the process of identifying highlights in live-streaming broadcasts using text analysis. This task can be very time-consuming, as it typically requires watching the entire video in order to identify the most notable or interesting moments. By automating this process, the creator of the broadcast can focus on creating content rather than sifting through hours of footage.

To begin the project, research was conducted with the goal of finding the best methods for analyzing chat messages in order to identify highlights. However, difficulties were encountered due to the international user base of the chosen streaming platform, Twitch. By instead focusing on the emotes used, it was possible to make a system that is indifferent to languages.

One of the primary goals of the project was to make the solution as scalable as possible. To achieve this, the system was implemented using a service-oriented system architecture and every implementation detail was carefully researched. Every system component was written using the programming language Rust, which is known for its low resource overhead.

The final system had four primary components: a WebSocket-based chat message collector, a service for analyzing the chat messages, a service for identifying potential highlights, and a service for cutting the original video into highlights. The chat message collector was responsible for gathering all of the messages sent in the chat during the broadcast, the analysis and highlight identification services then used this data to identify patterns or trends that might indicate a highlight. The service then analyzed these patterns and identified the most promising candidates for inclusion in a highlight reel. Finally, the video-cutting service took the identified highlights and created a new video file that included only those segments.

Overall, the system was successful in identifying a number of highlights that would normally require manual review of the entire video. Many of these highlights revolved around the streamer rather than the game itself, which is something that would not be possible using a computer vision-based model. This demonstrates the potential of using text analysis to automate the highlight identification process and allows creators to focus on creating content rather than sifting through hours of footage.
