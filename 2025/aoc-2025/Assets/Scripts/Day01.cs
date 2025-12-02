using UnityEngine;
using System.Collections.Generic;
using System;

public class Day01 : MonoBehaviour
{
    void Start()
    {
        var inputTextFile = Resources.Load<TextAsset>("Inputs/Day01");

        var turns = ReadTurns(inputTextFile);

        Part1(turns);
        Part2(turns);
    }

    List<int> ReadTurns(TextAsset inputTextFile)
    {
        var turns = new List<int>();
        var lines = inputTextFile.ToString().Split("\n");
        foreach (var line in lines)
        {
            if (line.Length == 0)
            {
                continue;
            }
            var turn = int.Parse(line[1..]);
            if (line[0] == 'L')
            {
                turn = -turn;
            }
            turns.Add(turn);
        }
        return turns;
    }

    void Part1(List<int> turns)
    {
        var zeroCount = 0;
        var dial = 50;
        foreach (var turn in turns)
        {
            dial = (dial + turn) % 100;
            if (dial == 0)
            {
                zeroCount++;
            }
        }
        Debug.Log("Part 1: " + zeroCount);
    }

    void Part2(List<int> turns)
    {
        var zeroCount = 0;
        var dial = 50;
        foreach (var turn in turns)
        {
            var turnCounter = Math.Abs(turn);
            if (turn < 0)
            {
                while (turnCounter > 0)
                {
                    dial--;
                    turnCounter--;
                    if (dial == 0)
                    {
                        zeroCount += 1;
                    }
                    else if (dial == -1)
                    {
                        dial = 99;
                    }
                }
            }
            else
            {
                while (turnCounter > 0)
                {
                    dial++;
                    turnCounter--;
                    if (dial == 100)
                    {
                        dial = 0;
                        zeroCount += 1;
                    }
                }
            }
        }
        Debug.Log("Part 2: " + zeroCount);
    }
}
