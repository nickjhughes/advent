param ([Parameter(Mandatory=$true)][string]$day)
echo "Running day $day"
docker build -t aoc-2024 --quiet --build-arg DAY="$day" . && docker run aoc-2024
