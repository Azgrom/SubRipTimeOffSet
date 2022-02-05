namespace Library;

internal struct TimeData
{
    private TimeData(ushort hours, ushort minutes, ushort seconds, ushort milliseconds)
    {
        Hours = hours;
        Minutes = minutes;
        Seconds = seconds;
        Milliseconds = milliseconds;
    }

    private TimeData(IReadOnlyList<string> timeVec)
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

    public static TimeData TimeSplitter(string timeStr) =>
        new TimeData(timeStr.Split(SplitParameter));

    public uint ConvertUnitsToMilliseconds()
    {
        var minutes = Minutes + Hours * 60;
        var seconds = Seconds + minutes * 60;
        var totalTimeMilliseconds = (uint) (Milliseconds + seconds * 1_000);

        return totalTimeMilliseconds;
    }

    public static TimeData ConvertMillisecondsToTimeData(uint millisecondsTimeData)
    {
        var milliseconds = Module(millisecondsTimeData, 1_000);
        var seconds = Module(millisecondsTimeData - milliseconds, 60_000) / 1_000;
        var minutes = Module(milliseconds - (seconds + milliseconds), 3_600_000) / 60_000;
        var hours = millisecondsTimeData - (minutes + seconds + milliseconds) / 3_600_000;

        return new TimeData((ushort) hours, (ushort) minutes, (ushort) seconds, (ushort) milliseconds);
    }

    private static uint Module(uint n, uint d) => n - d * (n / d);

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
