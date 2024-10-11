param ([Parameter(Mandatory=$true)][string]$day)
echo "Running day $day"
docker build -t aoc-2024 --build-arg DAY="$day" . && docker run -it aoc-2024
