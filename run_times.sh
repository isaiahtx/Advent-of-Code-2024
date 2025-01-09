#!/bin/bash

# Ensure we have the right number of arguments
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <command> <num_runs>"
  exit 1
fi

command="$1"  # The command to run
num_runs="$2" # Number of times to run the command

outputs=()                          # Array to store unique outputs
output_file="run_times_outputs.txt" # File to log all outputs
echo "" >"$output_file"             # Clear the output file

trap ctrl_c INT

function ctrl_c() {
  echo -e "\nCaught interrupt (Ctrl+C)!"
  echo "Number of distinct outputs so far: ${#outputs[@]}"
  exit 0
}

for i in $(seq 1 "$num_runs"); do
  output=$(eval "$command") # Execute the command and capture its output

  # Save output to file
  echo "Output $i:" >>$output_file
  echo "__________________________________________________" >>$output_file
  echo "$output" >>"$output_file"
  echo "__________________________________________________" >>$output_file
  echo >>$output_file

  # Check if the output is already seen
  seen=false
  for existing in "${outputs[@]}"; do
    if [[ "$existing" == "$output" ]]; then
      seen=true
      break
    fi
  done

  if [ "$seen" = false ]; then
    outputs+=("$output")
  fi

  # Print progress every 10 iterations
  if ((i % 10 == 0)); then
    echo "Completed $i iterations. Distinct outputs so far: ${#outputs[@]}"
  fi

done

printf "\nExecution completed.\n"
echo "Number of distinct outputs: ${#outputs[@]}"
