using UnityEngine;
using System.Collections.Generic;
using System.Linq;
using UnityEngine.AI;

public class Day02 : MonoBehaviour
{
    void Start()
    {
        var inputTextFile = Resources.Load<TextAsset>("Inputs/Day02");

        var ranges = LoadRanges(inputTextFile);

        Part1(ranges);
        Part2(ranges);
    }

    List<(long, long)> LoadRanges(TextAsset inputTextFile)
    {
        var ranges = new List<(long, long)>();
        var line = inputTextFile.ToString().TrimEnd();
        // var line = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        var rangeStrings = line.Split(",");
        foreach (var rangeString in rangeStrings)
        {
            var parts = rangeString.Split("-");
            var start = long.Parse(parts[0]);
            var end = long.Parse(parts[1]);
            ranges.Add((start, end));
        }
        return ranges;
    }

    void Part1(List<(long, long)> ranges)
    {
        var invalidIds = new List<long>();
        foreach (var (start, end) in ranges)
        {
            for (var id = start; id <= end; id++)
            {
                var idString = id.ToString();
                if (idString.Length % 2 != 0)
                {
                    continue;
                }
                if (idString[..(idString.Length / 2)] == idString[(idString.Length / 2)..])
                {
                    invalidIds.Add(id);
                }
            }
        }
        var answer = invalidIds.Sum();
        Debug.Log("Part 1: " + answer);
    }

    void Part2(List<(long, long)> ranges)
    {
        var invalidIds = new List<long>();
        foreach (var (start, end) in ranges)
        {
            for (var id = start; id <= end; id++)
            {
                var idString = id.ToString();
                for (var sequenceLength = 1; sequenceLength <= idString.Length / 2; sequenceLength++)
                {
                    if (idString.Length % sequenceLength != 0)
                    {
                        continue;
                    }
                    var sequence = idString[..sequenceLength];
                    var isInvalid = true;
                    for (var i = 0; i < idString.Length; i += sequenceLength)
                    {
                        if (sequence != idString[i..(i + sequenceLength)])
                        {
                            isInvalid = false;
                        }
                    }
                    if (isInvalid)
                    {
                        invalidIds.Add(id);
                        break;
                    }
                }
            }
        }
        var answer = invalidIds.Sum();
        Debug.Log("Part 2: " + answer);
    }
}
