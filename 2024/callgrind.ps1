param ([Parameter(Mandatory=$true)][string]$day)
echo "Callgrind day $day"
docker build -t aoc-2024 --build-arg DAY="$day" -f Dockerfile-callgrind . && docker run -v .:/parent -it aoc-2024
