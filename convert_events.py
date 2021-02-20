#!/usr/bin/env python3

with open("recorded_events.txt", "r") as fil:
    text = ""
    for line in fil:
        if not line.startswith("    "):
            print(text + '"""}')
            text = 'yield {"args": ["' + line[:-1] + '"], "stdin": """'
        else:
            text += line[4:-1] + "\\n"