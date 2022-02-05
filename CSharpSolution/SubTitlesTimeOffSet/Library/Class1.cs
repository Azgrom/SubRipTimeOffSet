using System.Text.RegularExpressions;

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
    private TimeStamp(TimeData start, TimeData end)
    {
        Start = start;
        End = end;
    }

    public TimeData Start { get; set; }

    public TimeData End { get; set; }

    public static TimeStamp Parser(List<string> patternsWrapper)
    {
        if (patternsWrapper.Count != 2) throw new Exception("Panic");
        
        var startEndTimes = patternsWrapper[1].Split(SplitParameter);
        return new TimeStamp(TimeData.TimeSplitter(startEndTimes[0]), TimeData.TimeSplitter(startEndTimes[1]));
    }

    public TimeStamp Offset(UInt32 offset)
    {
        var startInMilliseconds = Start.ConvertUnitsToMilliseconds() + offset;
        var endInMilliseconds = End.ConvertUnitsToMilliseconds() + offset;
        
        return new TimeStamp(TimeData.ConvertMillisecondsToTimeData(startInMilliseconds), TimeData.ConvertMillisecondsToTimeData(endInMilliseconds));
    }

    private const string SplitParameter = " --> ";
}

internal struct SubRipContent
{
    private SubRipContent(TimeStamp dialogTiming, string dialogString)
    {
        DialogTiming = dialogTiming;
        DialogString = dialogString;
    }

    public TimeStamp DialogTiming { get; set; }
    public string DialogString { get; set; }

    private string DialogParser(IEnumerable<string> patternWrapper) =>
        patternWrapper.Where(dialogLine => dialogLine != "")
            .Aggregate<string, string>(null!, (current, dialogLine) =>
                string.Join(current, dialogLine));

    public List<SubRipContent> SubRipParser(string textfile_content)
    {
        var patternWrapper = new List<string>();
        var subRipVec = new List<SubRipContent>();

        foreach (var subRipLine in Regex.Split(textfile_content, "\r\n|\r|\n"))
        {
            patternWrapper.Add(subRipLine);

            if (subRipLine != "") continue;
            var dialogTiming = TimeStamp.Parser(patternWrapper);
            var dialogString = DialogParser(patternWrapper);
            
            patternWrapper.Clear();
                
            subRipVec.Add(new SubRipContent(dialogTiming, dialogString));
        }

        return subRipVec;
    }
}
