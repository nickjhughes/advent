param ([Parameter(Mandatory=$true)][string]$day)
echo "Debugging day $day"
docker build -t aoc-2024 --build-arg DAY="$day" -f Dockerfile-gdbfrontend . && docker run -p 5550:5550 -it aoc-2024
