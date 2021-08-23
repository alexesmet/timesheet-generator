# timesheet-generator
Timesheet in my format goes in, timesheet in CSV format - out. Let me demonstrate.

Consider that I have a file `timesheet.time` in a following format:
```
29.06.2021
CC-2216
20m Setting up gitlab authentication
15m Test connectio and endpoint behaviour
15m Add method to js api clint
20m Talk

30.06.2021
CC-2216
10m Sey up gitlab verification, connect 2-factor auth
20m Send answers with
20m Disable button until
10m Trying to style disabled button

02.07.2021
CC-2216
45m Refactoring on frontend
30m Writing sql-quesry for last personal
30m Clashing with gradle build
```

Running `timesheet-generator timesheet.time` or `cat timesheet.time | timesheet-generator` generates the following in csv:

|date      |time|hours|label  |description                                      |
|----------|----|-----|-------|-------------------------------------------------|
|29.06.2021|20m |0.33 |CC-2216|Setting up gitlab authentication                 |
|29.06.2021|15m |0.25 |CC-2216|Test connectio and endpoint behaviour            |
|29.06.2021|15m |0.25 |CC-2216|Add method to js api clint                       |
|29.06.2021|20m |0.33 |CC-2216|Talk                                             |
|30.06.2021|10m |0.16 |CC-2216|Sey up gitlab verification, connect 2-factor auth|
|30.06.2021|20m |0.33 |CC-2216|Send answers with                                |
|30.06.2021|20m |0.33 |CC-2216|Disable button until                             |
|30.06.2021|10m |0.16 |CC-2216|Trying to style disabled button                  |
|02.07.2021|45m |0.75 |CC-2216|Refactoring on frontend                          |
|02.07.2021|30m |0.5  |CC-2216|Writing sql-quesry for last personal             |
|02.07.2021|30m |0.5  |CC-2216|Clashing with gradle build                       |

