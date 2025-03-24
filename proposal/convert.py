import json
import re

def get_parentheses_content(text):
  """Extracts and returns the content inside parentheses using regex."""
  pattern = r"\((.*?)\)"  # Matches anything inside parentheses
  matches = re.findall(pattern, text)
  return matches

with open("toconv.txt") as f:
    state = 0
    valid_op = False
    output = {}
    all_types = []
    t = {"opcodes": []}
    for l in f.readlines():
        if l.startswith("Type"):
            valid_op = False
            all_types.append(t)
            t = {
                "name": get_parentheses_content(l)[0],
                "opcodes": [],
            }
        elif l.startswith("OPCODE"):
            if valid_op:
                output["bits"] = []
                t["opcodes"].append(output)
            valid_op = True
            output = {}
            output["name"] = get_parentheses_content(l)[0]
            state = 1
        elif state == 1:
            output["pneumonic"] = l
            state = 2
        elif state == 2:
            if "description" in output:
                output["description"] += "\n" + l
            else:
                output["description"] = l
    all_types.append(t)
    print(json.dumps(all_types))