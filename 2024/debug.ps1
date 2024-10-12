param ([Parameter(Mandatory=$true)][string]$day)
echo "Debugging day $day"
docker build -t aoc-2024 --build-arg DAY="$day" . && docker run -it --entrypoint gdb aoc-2024 day$day
