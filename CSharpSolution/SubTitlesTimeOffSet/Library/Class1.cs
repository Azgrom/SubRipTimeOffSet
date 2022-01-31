namespace Library;


public struct TimeData
{
    public TimeData(IReadOnlyList<string> timeVec)
    {
        Hours = Convert.ToUInt16(timeVec[0]);
        Minutes = Convert.ToUInt16(timeVec[1]);
        Seconds = Convert.ToUInt16(timeVec[2]);
        Milliseconds = Convert.ToUInt16(timeVec[3]);
    }
        
    public ushort Hours { get; set; }
    public ushort Minutes { get; set; }
    public ushort Seconds { get; set; }
    public ushort Milliseconds { get; set; }

    public TimeData TimeSplitter(string timeStr) =>
        new TimeData(timeStr.Split(SplitParameter));

    public uint ConvertUnitsToMilliseconds()
    {
        var minutes = Minutes + Hours * 60;
        var seconds = Seconds + minutes * 60;
        var totalTimeMilliseconds = (uint) (Milliseconds + seconds * 1_000);

        return totalTimeMilliseconds;
    }

    public void ConvertMillisecondsToTimeData(uint millisecondsTimeData)
    {
        var timeSpan = TimeSpan.FromMilliseconds(millisecondsTimeData);
        Hours;
    }

    public void Set(TimeSpan? timeSpan = null,
        ushort? hours = null,
        ushort? minutes = null,
        ushort? seconds = null,
        ushort? milliseconds = null)
    {
        if (timeSpan != null)
        {
            Hours = timeSpan.Hours;

        }
    }

    private static readonly char[] SplitParameter = {':', ','};
}

internal struct TimeStamp
{
    public TimeData Start { get; set; }
    public TimeData End { get; set; }
}

internal struct SubRipContent
{
    public TimeStamp DialogTiming { get; set; }
    public string DialogString { get; set; }
}
