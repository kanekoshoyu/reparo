# reparo (for streaming backtest data)

life is like a chocolate, you never know what you get next.  
That's not the case for your event-driven project with reparo. let's rewind the events for backtesting.

it does 2 things: 
- subscribe and store streaming data into a file
- load streaming data from a file and publish

the only thing we care about whether the data can be repeated is the timestamp. therefore the struct to be repeated should have a timestamp.  
in this project, this is encapsulated as trait called "rewindable"  
the file should be human readable and data should be repeatable, so that we can manually edit for "data cleansing"  