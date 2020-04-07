use chrono;

#[derive(Debug, Copy, Clone)]
enum DayE {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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
    exceptions: Vec<(ScheduleException, Option<Range>)>,
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

struct Scheduler {
    most_popular_days: [u32; 7],
    i_schedules: Vec<InstructorSchedule>,
}

impl Scheduler {
    fn new(schedules: Vec<InstructorSchedule>) -> Self {
        let mut days: [u32; 7] = Default::default();

        Scheduler {
            most_popular_days: days,
            i_schedules: schedules,
        }
    }
    fn resolve(mut self) {
        unimplemented!()
    }
    fn find_most_popular_day(&mut self) {
        let mut day = 0;
        let mut incl = false;
        for (i, inst_sched) in (&self.i_schedules).iter().enumerate() {
            for (e, r) in &inst_sched.exceptions {
                if let Some(_r) = r {
                    match _r {
                        Range::Inclusive => incl = true,
                        _ => incl = false,
                    }
                };
                match e {
                    ScheduleException::Day(e) => {
                        if incl {
                            self.most_popular_days = get_day_uint(*e, self.most_popular_days);
                        }
                    }
                    ScheduleException::TimeDate(e) => {
                        if incl {
                            self.most_popular_days = get_day_uint(e.0, self.most_popular_days);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn get_day_uint(day: DayE, mut pop_days: [u32; 7]) -> [u32; 7] {
    match day {
        DayE::Monday => pop_days[0] += 1,
        DayE::Tuesday => pop_days[1] += 1,
        DayE::Wednesday => pop_days[2] += 1,
        DayE::Thursday => pop_days[3] += 1,
        DayE::Friday => pop_days[4] += 1,
        DayE::Saturday => pop_days[5] += 1,
        DayE::Sunday => pop_days[6] += 1,
    }
    pop_days
}

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
                Some(Range::Inclusive),
            ),
            (
                ScheduleException::TimeDate(TimeDateE(
                    DayE::Thursday,
                    (
                        chrono::NaiveTime::from_hms(10, 0, 0),
                        chrono::Duration::hours(5),
                    ),
                )),
                Some(Range::Inclusive),
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
            Some(Range::Inclusive),
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

    #[test]
    fn test_pop_days() {
        let mut pdays: [u32; 7] = Default::default();

        pdays = get_day_uint(DayE::Monday, pdays);
        assert_eq!(pdays[0], 1);
        pdays = get_day_uint(DayE::Monday, pdays);
        assert_eq!(pdays[0], 2);
        pdays = get_day_uint(DayE::Sunday, pdays);
        assert_eq!(pdays[6], 1);
    }

    #[test]
    fn test_find_pop_days() {
        let mut scheduler = Scheduler::new(setup());
        scheduler.find_most_popular_day();
        println!("popular days: {:?}", scheduler.most_popular_days);
    }
}
