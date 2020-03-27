use chrono;

enum DayE {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

struct TimeDateE(DayE, TimeRange);

enum MeetingDays {
    MWF,
    MTWRF,
    TR,
    MW,
    FSa,
    M,
    T,
    W,
    R,
    F,
    Sa,
    Su,
}

enum InstructorT {
    Tenured,
    TTrack,
    NonTenured,
    PTAdjunct,
    FTAdjunct,
}

enum Cause {
    Preference,
    Medical,
    Family,
}

type TimeRange = (chrono::NaiveTime, chrono::Duration);

enum ScheduleException {
    Day(DayE),
    Time(TimeRange),
    TimeDate(TimeDateE),
}

enum Range {
    Inclusive,
    Exclusive,
}

enum ScheduleCondition {
    SequentialC,
    OverlapC,
    SessionC,
}

struct InstructorSchedule {
    instructor: Instructor,
    exceptions: Vec<(ScheduleException, Range)>,
    conditions: Vec<ScheduleCondition>,
    classes: Class,
}

enum SequentialC {
    Continuous,
    NonContinuous,
}

type OverlapC = Option<Instructor>;

enum SessionC {
    Daytime,
    Evening,
}

struct Class {
    days: MeetingDays,
    duration: usize,
}

struct Instructor {
    i_type: InstructorT,
    f_name: Option<String>,
    l_name: String,
}
