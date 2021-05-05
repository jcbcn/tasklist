use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Task {
    pub id: Option<u64>,
    pub name: String,
    pub due: Option<NaiveDateTime>,
    pub completed: bool
}

// enum Due {
//     Td,
//     Today,
//     Tm,
//     Tomorrow,

//     Tw,
//     ThisWeek,
//     Nw,
//     NextWeek,

//     Mon,
//     Monday,
//     Tue,
//     Tuesday,
//     Wed,
//     Wednesday,
//     Thur,
//     Thursday,
//     Fri,
//     Friday,
//     Sat,
//     Saturday,
//     Sun,
//     Sunday,
// }