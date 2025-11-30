using UnityEngine;
using System.Collections.Generic;
using System;

public class Day01 : MonoBehaviour
{
    void Start()
    {
        var inputTextFile = Resources.Load<TextAsset>("Inputs/Day01");

        var (leftNumbers, rightNumbers) = ReadNumbers(inputTextFile);
        leftNumbers.Sort();
        rightNumbers.Sort();

        Part1(leftNumbers, rightNumbers);
        Part2(leftNumbers, rightNumbers);
    }

    (List<int>, List<int>) ReadNumbers(TextAsset inputTextFile)
    {
        var leftNumbers = new List<int>();
        var rightNumbers = new List<int>();
        var lines = inputTextFile.ToString().Split("\n");
        foreach (var line in lines)
        {
            if (line.Length == 0)
            {
                continue;
            }
            var parts = line.Split("   ");
            leftNumbers.Add(int.Parse(parts[0]));
            rightNumbers.Add(int.Parse(parts[1]));
        }
        return (leftNumbers, rightNumbers);
    }

    void Part1(List<int> leftNumbers, List<int> rightNumbers)
    {
        var totalDifference = 0;
        for (var i = 0; i < leftNumbers.Count; i++)
        {
            var difference = Math.Abs(leftNumbers[i] - rightNumbers[i]);
            totalDifference += difference;
        }
        Debug.Log("Part 1: " + totalDifference);
    }

    void Part2(List<int> leftNumbers, List<int> rightNumbers)
    {
        var totalSimilarity = 0;
        foreach (var left in leftNumbers)
        {
            var count = 0;
            foreach (var right in rightNumbers)
            {
                if (left == right)
                {
                    count += 1;
                }
            }
            totalSimilarity += left * count;
        }
        Debug.Log("Part 2: " + totalSimilarity);
    }
}
