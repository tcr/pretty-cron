// https://github.com/azza-bazoo/prettycron/blob/master/prettycron.js

extern crate cron;

use cron::{Schedule, OrdinalSet};
use cron::time_unit::*;
use std::str::FromStr;

/*
* For an array of numbers, e.g. a list of hours in a schedule,
* return a string listing out all of the values (complete with
* "and" plus ordinal text on the last item).
*/
fn number_list(numbers: &OrdinalSet) -> String {
    if numbers.len() < 2 {
        // TODO  return moment()._locale.ordinal(numbers);
        return format!("{}", numbers.iter().nth(0).unwrap());
    }

    let mut nums: Vec<_> = numbers.iter().cloned().map(|x| x.to_string()).collect();
    let last_val = nums.pop().unwrap();
    // TODO ordinal for last
    return format!("{} and {}", nums.join(", "), last_val);
}

fn step_size(numbers: &OrdinalSet) -> usize {
    if numbers.len() <= 1 {
        return 0;
    }

    let expectedStep = numbers.iter().nth(1).unwrap() - numbers.iter().nth(0).unwrap();
    if numbers.len() == 2 {
        return expectedStep as usize;
    }

    return 0;
    // Check that every number is the previous number + the first number
    //return numbers.slice(1).every(function(n,i,a){
    //  return (i === 0 ? n : n-a[i-1]) === expectedStep;
    //}) ? expectedStep : 0;
}

fn isEveryOther(step: usize, numbers: &OrdinalSet) -> bool {
    return numbers.len() == 30 && step == 2;
}

fn isTwicePerHour(step: usize, numbers: &OrdinalSet) -> bool {
    return numbers.len() == 2 && step == 30;
}

fn isOnTheHour(numbers: &OrdinalSet) -> bool {
    return numbers.len() == 1 && *numbers.iter().nth(0).unwrap() == 0;
}

fn isStepValue(step: usize, numbers: &OrdinalSet) -> bool {
    // Value with slash (https://en.wikipedia.org/wiki/Cron#Non-Standard_Characters)
    return numbers.len() > 2 && step > 0;
}

/*
* For an array of numbers of seconds, return a string
* listing all the values unless they represent a frequency divisible by 60:
* /2, /3, /4, /5, /6, /10, /12, /15, /20 and /30
*/
fn getMinutesTextParts(minutes: &Minutes) -> (String, String) {
    if *minutes == Minutes::all() {
        return ("minute".to_string(), "".to_string());
    }

    let numbers = minutes.ordinals();
    let step = step_size(numbers);

    return if isOnTheHour(numbers) {
        ("".to_string(), "hour, on the hour".to_string())
    } else if isEveryOther(step, numbers) {
        ("other minute".to_string(), "".to_string())
    } else if isStepValue(step, numbers) {
        ("".to_string(), format!("{} minutes", step))
    } else if isTwicePerHour(step, numbers) {
        ("".to_string(), "first and 30th minute".to_string())
    } else {
        ("".to_string(), format!("{} minute", number_list(numbers)))
    };
}

/*
* For an array of numbers of seconds, return a string
* listing all the values unless they represent a frequency divisible by 60:
* /2, /3, /4, /5, /6, /10, /12, /15, /20 and /30
*/
fn getSecondsTextParts(numbers: &Seconds) -> (String, String) {
    let step = step_size(numbers.ordinals());
    if numbers.ordinals().len() == 0 {
        return ("second".to_string(), "".to_string());
    }
    if isEveryOther(step, numbers.ordinals()) {
        return ("".to_string(), "other second".to_string());
    } else if isStepValue(step, numbers.ordinals()) {
        return ("".to_string(), format!("{} seconds", step));
    } else {
        return ("minute".to_string(), format!("starting on the {}", if numbers.ordinals().len() == 2 && step == 30 { "first and 30th second".to_string() } else { format!("{} second", number_list(numbers.ordinals())) }));
    }
}

/*
* Parse a number into day of week, or a month name;
* used in date_list below.
*/
#[derive(Copy, Clone)]
enum DateNaming {
    DOW,
    MON
}

fn numberToDateName(value: u32, kind: DateNaming) -> String {
    return match kind {
        DateNaming::DOW => {
            format!("DAY({})", value - 1)
            //TODO return moment().day(value - 1).format('ddd')
        }
        DateNaming::MON => {
            format!("MONTH({})", value - 1)
        }
    };
}

/*
* From an array of numbers corresponding to dates (given in type: either
* days of the week, or months), return a string listing all the values.
*/
fn date_list(numbers: &OrdinalSet, kind: DateNaming) -> String {
    let mut values: Vec<_> = numbers.iter().cloned().collect();

    if values.len() < 2 {
        return numberToDateName(values[0], kind);
    }

    let last_val = values.pop().unwrap();
    let mut output_text = "".to_string();

    for item in values {
        if output_text.len() > 0 {
            output_text.push_str(", ");
        }
        output_text.push_str(&numberToDateName(item, kind));
    }
    return format!("{} and {}", output_text, numberToDateName(last_val, kind));
}

/// Given a schedule from later.js (i.e. after parsing the cronspec),
/// generate a friendly sentence description.
pub fn prettify_cron(expression: &str, use_seconds: bool) -> String {
    let schedule = Schedule::from_str(expression).unwrap();

    let mut textParts = vec![];

    let every_second = use_seconds && (schedule.seconds == Seconds::all());
    let every_minute = schedule.minutes == Minutes::all();
    let every_hour = schedule.hours == Hours::all();
    let every_weekday = schedule.days_of_week == DaysOfWeek::all();
    let every_day_in_month = schedule.days_of_month == DaysOfMonth::all();
    let every_month = schedule.months == Months::all();

    let oneOrTwoSecondsPerMinute = schedule.seconds.ordinals().len() <= 2;
    let oneOrTwoMinutesPerHour = schedule.minutes.ordinals().len() <= 2;
    let oneOrTwoHoursPerDay = schedule.hours.ordinals().len() <= 2;
    let onlySpecificDaysOfMonth = schedule.days_of_month.ordinals().len() != 31;

    if oneOrTwoHoursPerDay && oneOrTwoMinutesPerHour && oneOrTwoSecondsPerMinute {
        // If there are only one or two specified values for
        // hour or minute, print them in HH:MM format, or HH:MM:ss if seconds are used
        // If seconds are not used, later.js returns one element for the seconds (set to zero)
    } else {
        let seconds = getSecondsTextParts(&schedule.seconds);
        let minutes = getMinutesTextParts(&schedule.minutes);
        let mut beginning = "".to_string();
        let mut end = "".to_string();

        textParts.push("Every".to_string());

        // Otherwise, list out every specified hour/minute value.
        let hasSpecificSeconds =
            (schedule.seconds.ordinals().len() > 1
                && schedule.seconds.ordinals().len() < 60)
            || (schedule.seconds.ordinals().len() == 1
                && *schedule.seconds.ordinals().iter().nth(0).unwrap() != 0);
        if hasSpecificSeconds {
          beginning = seconds.0.to_string();
          end = seconds.1.to_string();
        }

        if !every_hour {
            if hasSpecificSeconds {
                end.push_str(" on the ");
            }
            if !every_minute { // and only at specific minutes
                let hours = format!("{} hour", number_list(schedule.hours.ordinals()));
                if !hasSpecificSeconds && isOnTheHour(schedule.minutes.ordinals()) {
                    textParts = vec!["On the".to_string()];
                    end.push_str(&hours);
                } else {
                    beginning = minutes.0.to_string();
                    end.push_str(&format!("{} past the {}", minutes.1, hours));
                }
            } else { // specific hours, but every minute
                end.push_str(&format!("minute of {} hour", number_list(schedule.hours.ordinals())));
            }
        } else if !every_minute { // every hour, but specific minutes
          beginning = minutes.0.to_string();
          end.push_str(&minutes.1);
          if !isOnTheHour(schedule.minutes.ordinals())
            && (onlySpecificDaysOfMonth || !every_weekday || !every_month) {
            end.push_str(" past every hour");
          }
        } else if every_second && !every_minute {
          beginning = seconds.0.to_string();
        } else if !use_seconds || !hasSpecificSeconds { // cronspec has "*" for both hour and minute
          beginning.push_str(&minutes.0);
        }

        textParts.push(beginning);
        textParts.push(end);
    }

    if onlySpecificDaysOfMonth { // runs only on specific day(s) of month
        textParts.push(format!("on the {}", number_list(schedule.days_of_month.ordinals())));
        if every_month {
            textParts.push("of every month".into());
        }
    }

    if !every_weekday { // runs only on specific day(s) of week
        if every_day_in_month {
            // if both day fields are specified, cron uses both; superuser.com/a/348372
            textParts.push("and every".into());
        } else {
            textParts.push("on".into());
        }
        textParts.push(date_list(schedule.days_of_week.ordinals(), DateNaming::DOW));
    }

    if every_month {
        textParts.push("day of every month".into());
    } else {
        // runs only in specific months; put this output last
        textParts.push(format!("in {}", date_list(schedule.months.ordinals(), DateNaming::MON)));
    }

    return textParts
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>()
        .join(" ");
}

/*

     * Given a schedule from later.js (i.e. after parsing the cronspec),
     * generate a friendly sentence description.
     *
    var scheduleToSentence = function(schedule, useSeconds) {
      var textParts = [];

      var oneOrTwoSecondsPerMinute = schedule['s'] && schedule['s'].length <= 2;
      var oneOrTwoMinutesPerHour = schedule['m'] && schedule['m'].length <= 2;
      var oneOrTwoHoursPerDay = schedule['h'] && schedule['h'].length <= 2;
      var onlySpecificDaysOfMonth = schedule['D'] && schedule['D'].length !== 31;
      if ( oneOrTwoHoursPerDay && oneOrTwoMinutesPerHour && oneOrTwoSecondsPerMinute ) {
        // If there are only one or two specified values for
        // hour or minute, print them in HH:MM format, or HH:MM:ss if seconds are used
        // If seconds are not used, later.js returns one element for the seconds (set to zero)

        var hm = [];
        var m = moment();
        for (var i=0; i < schedule['h'].length; i++) {
          for (var j=0; j < schedule['m'].length; j++) {
            for (var k=0; k < schedule['s'].length; k++) {
              m.hour(schedule['h'][i]);
              m.minute(schedule['m'][j]);
              m.second(schedule['s'][k]);
              hm.push(m.format( useSeconds ? 'HH:mm:ss' : 'HH:mm'));
            }
          }
        }
        if (hm.length < 2) {
          textParts.push( hm[0] );
        } else {
          var last_val = hm.pop();
          textParts.push( hm.join(', ') + ' and ' + last_val );
        }
        if (everyWeekday && everyDayInMonth) {
          textParts.push('every day');
        }

      } else {
        var seconds = getSecondsTextParts(schedule['s']);
        var minutes = getMinutesTextParts(schedule['m']);
        var beginning = '';
        var end = '';

        textParts.push('Every');

        // Otherwise, list out every specified hour/minute value.
        var hasSpecificSeconds = schedule['s'] && (
            schedule['s'].length > 1 && schedule['s'].length < 60 ||
            schedule['s'].length === 1 && schedule['s'][0] !== 0 );
        if(hasSpecificSeconds) {
          beginning = seconds.beginning;
          end = seconds.text;
        }

        if(schedule['h']) { // runs only at specific hours
          if( hasSpecificSeconds ) {
            end += ' on the ';
          }
          if (schedule['m']) { // and only at specific minutes
            var hours = numberList(schedule['h']) + ' hour';
            if( !hasSpecificSeconds && isOnTheHour(schedule['m']) ) {
              textParts = [ 'On the' ];
              end += hours;
            } else {
              beginning = minutes.beginning;
              end += minutes.text + ' past the ' + hours;
            }
          } else { // specific hours, but every minute
            end += 'minute of ' + numberList(schedule['h']) + ' hour';
          }
        } else if(schedule['m']) { // every hour, but specific minutes
          beginning = minutes.beginning;
          end += minutes.text;
          if( !isOnTheHour(schedule['m']) && ( onlySpecificDaysOfMonth || schedule['d'] || schedule['M'] ) ) {
            end += ' past every hour';
          }
        } else if( !schedule['s'] && !schedule['m'] ) {
          beginning = seconds.beginning;
        } else if( !useSeconds || !hasSpecificSeconds) { // cronspec has "*" for both hour and minute
          beginning += minutes.beginning;
        }
        textParts.push(beginning);
        textParts.push(end);
      }

      if (onlySpecificDaysOfMonth) { // runs only on specific day(s) of month
        textParts.push('on the ' + numberList(schedule['D']));
        if (!schedule['M']) {
          textParts.push('of every month');
        }
      }

      if (schedule['d']) { // runs only on specific day(s) of week
        if (schedule['D']) {
          // if both day fields are specified, cron uses both; superuser.com/a/348372
          textParts.push('and every');
        } else {
          textParts.push('on');
        }
        textParts.push(dateList(schedule['d'], 'dow'));
      }

      if (schedule['M']) {
        if( schedule['M'].length === 12 ) {
          textParts.push('day of every month');
        } else {
          // runs only in specific months; put this output last
          textParts.push('in ' + dateList(schedule['M'], 'mon'));
        }
      }

      return textParts.filter(function(p) { return p; }).join(' ');
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //               sec  min   hour   day of month   month   day of week   year
        //let expression = "*   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
        //let res = super::prettify_cron(expression, true);
        //println!("{:?}", res);

        // Every hour, on the hour.
        let expression = "0 * * * * * *";
        let res = super::prettify_cron(expression, false);
        println!("{:?}", res);
    }
}
