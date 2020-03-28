use chrono;

#[derive(Debug)]
enum DayE {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
struct TimeDateE(DayE, TimeRange);

#[derive(Debug)]
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

#[derive(Debug, Clone)]
enum InstructorT {
    Tenured,
    TTrack,
    NonTenured,
    PTAdjunct,
    FTAdjunct,
}

#[derive(Debug)]
enum Cause {
    Preference,
    Medical,
    Family,
}

type TimeRange = (chrono::NaiveTime, chrono::Duration);

#[derive(Debug)]
enum ScheduleException {
    Day(DayE),
    Time(TimeRange),
    TimeDate(TimeDateE),
}

#[derive(Debug)]
enum Range {
    Inclusive,
    Exclusive,
}

#[derive(Debug)]
enum ScheduleCondition {
    Sequential(SequentialC),
    Overlap(Instructor),
    Session(SessionC),
    MaxDays(u8),
}

#[derive(Debug)]
struct InstructorSchedule {
    instructor: Instructor,
    exceptions: Vec<(ScheduleException, Range)>,
    conditions: Vec<ScheduleCondition>,
    classes: Vec<Class>,
}

#[derive(Debug)]
enum SequentialC {
    Continuous,
    NonContinuous,
}

#[derive(Debug)]
enum SessionC {
    Daytime,
    Evening,
}

#[derive(Debug)]
struct Class {
    name: String,
    duration: Option<usize>,
    // TODO: Re-add
    // c_type: ClassT,
    // TODO: add
    // component: ClassComponent
    // mode: ClassMode
}

#[derive(Debug)]
enum ClassT {
    Undergrad,
    Graduate,
}

#[derive(Debug)]
enum ClassComponent {
    Lecture,
    IndependentStudy,
    Lab,
    Internship,
}

#[derive(Debug)]
enum ClassMode {
    Online,
    InPerson,
}

#[derive(Debug)]
struct ClassDateTime(MeetingDays, TimeRange);

#[derive(Debug, Clone)]
struct Instructor {
    i_type: InstructorT,
    f_name: Option<String>,
    l_name: String,
}

type Slot = (Instructor, Class, ClassDateTime);
type Conflict = (Slot, Slot);
// struct ScheduleConflict(Conflict, TimeRange);
// type ScheduleResult = Result<Vec<ScheduleSlot>, Vec<ScheduleConflict>>;

type MWFDaySlots = [Option<[Option<Slot>; 3]>; 9];
type TRDaySlots = [Option<[Option<Slot>; 3]>; 6];
type TRNightSlots = [Option<[Option<Slot>; 3]>; 4];
type MWNightSlots = [Option<[Option<Slot>; 3]>; 4];
type SingleSlots = [Option<Slot>; 7];

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Vec<InstructorSchedule> {
        let mut prof_brown = Instructor {
            f_name: None,
            l_name: "brown".to_string(),
            i_type: InstructorT::Tenured,
        };
        let mut prof_purple = Instructor {
            f_name: None,
            l_name: "purple".to_string(),
            i_type: InstructorT::Tenured,
        };
        let mut prof_blue = Instructor {
            f_name: None,
            l_name: "blue".to_string(),
            i_type: InstructorT::NonTenured,
        };
        let mut prof_green = Instructor {
            f_name: None,
            l_name: "green".to_string(),
            i_type: InstructorT::Tenured,
        };

        let mut brown_exceptions = vec![
            (
                ScheduleException::TimeDate(TimeDateE(
                    DayE::Tuesday,
                    (
                        chrono::NaiveTime::from_hms(10, 0, 0),
                        chrono::Duration::hours(5),
                    ),
                )),
                Range::Inclusive,
            ),
            (
                ScheduleException::TimeDate(TimeDateE(
                    DayE::Thursday,
                    (
                        chrono::NaiveTime::from_hms(10, 0, 0),
                        chrono::Duration::hours(5),
                    ),
                )),
                Range::Inclusive,
            ),
        ];
        let mut brown_conds = vec![ScheduleCondition::Overlap(prof_purple.clone())];
        let mut brown_classes = vec![
            Class {
                name: "CS1000".to_string(),
                duration: None,
            },
            Class {
                name: "CS2000".to_string(),
                duration: None,
            },
        ];

        let mut purple_exceptions = vec![];
        let mut purple_conds = vec![
            ScheduleCondition::Overlap(prof_brown.clone()),
            ScheduleCondition::Sequential(SequentialC::Continuous),
            ScheduleCondition::MaxDays(2),
        ];
        let mut purple_classes = vec![
            Class {
                name: "CS6000".to_string(),
                duration: None,
            },
            Class {
                name: "CS6500".to_string(),
                duration: None,
            },
        ];

        let mut blue_exceptions = vec![(
            ScheduleException::Time((
                chrono::NaiveTime::from_hms(10, 0, 0),
                chrono::Duration::hours(4),
            )),
            Range::Inclusive,
        )];
        let mut blue_conds = vec![ScheduleCondition::Overlap(prof_green.clone())];
        let mut blue_classes = vec![
            Class {
                name: "CS100".to_string(),
                duration: None,
            },
            Class {
                name: "CS200".to_string(),
                duration: None,
            },
            Class {
                name: "CS300".to_string(),
                duration: None,
            },
            Class {
                name: "CS400".to_string(),
                duration: None,
            },
        ];

        let mut green_exceptions = vec![];
        let mut green_conds = vec![ScheduleCondition::Overlap(prof_blue.clone())];
        let mut green_classes = vec![
            Class {
                name: "ECE1000".to_string(),
                duration: None,
            },
            Class {
                name: "ECE2000".to_string(),
                duration: None,
            },
        ];

        vec![
            InstructorSchedule {
                instructor: prof_brown,
                exceptions: brown_exceptions,
                conditions: brown_conds,
                classes: brown_classes,
            },
            InstructorSchedule {
                instructor: prof_purple,
                exceptions: purple_exceptions,
                conditions: purple_conds,
                classes: purple_classes,
            },
            InstructorSchedule {
                instructor: prof_blue,
                exceptions: blue_exceptions,
                conditions: blue_conds,
                classes: blue_classes,
            },
            InstructorSchedule {
                instructor: prof_green,
                exceptions: green_exceptions,
                conditions: green_conds,
                classes: green_classes,
            },
        ]
    }

    #[test]
    fn test() {
        let v: MWFDaySlots = Default::default();
        println!("MWFDaySlots default: {:?}", v);
    }
}
