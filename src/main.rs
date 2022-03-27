mod note;

use hound;
use note::{ Name, Note };
use std::f32::consts::PI;
use std::collections::HashMap;

fn get_sample(frame: f32, frequency: f32) -> f32
{
    (2.0 * PI * frame * frequency / 44100.0).sin()
}

fn validate_first_species(cantus: &Vec<Note>, counter: &Vec<Note>, intervals: &Vec<f32>, next: Note, step: usize) -> bool
{
    if next.to_number_full() <= cantus[step].to_number_full()
    {
        return false;
    }

    if (next.to_number() == 6.0 && counter[step - 1].to_number() == 3.0) || (next.to_number() == 3.0 && counter[step - 1].to_number() == 6.0)
    {
        return false;
    }

    let current_interval_full = next - cantus[step];
    let current_interval = current_interval_full % 7.0 + 1.0;

    if current_interval == 2.0 || current_interval == 4.0 || current_interval == 7.0
    {
        return false;
    }

    if (current_interval == 5.0 && intervals[step - 1] == 5.0) || (current_interval == 1.0 && intervals[step - 1] == 1.0)
    {
        return false;
    }

    if current_interval_full > 10.0
    {
        return false;
    }

    if step > 2 && (current_interval == 3.0 || current_interval == 6.0) && current_interval == intervals[step - 1] && intervals[step - 1] == intervals[step - 2] && intervals[step - 2] == intervals[step - 3]
    {
        return false;
    }

    if step > 1
    {
        let dist = (next - counter[step - 1]) % 7.0;
        let last_dist = (counter[step - 1] - counter[step - 2]) % 7.0;

        if dist == -5.0
        {
            return false;
        }

        if (next - counter[step - 1]).abs() > 5.0
        {
            return false;
        }

        if last_dist > 2.0 && (dist < -2.0 || dist > -1.0)
        {
            return false;
        }

        if last_dist < -2.0 && (dist > 2.0 || dist < 1.0)
        {
            return false;
        }

        let cantus_dist = (cantus[step] - cantus[step - 1]) % 7.0;

        if (current_interval == 5.0 || current_interval == 1.0) && dist.signum() == cantus_dist.signum()
        {
            return false;
        }
    }

    true
}

fn counterpoint_first_species(cantus: &Vec<Note>, counter: &mut Vec<Note>, intervals: &mut Vec<f32>, step: usize) -> Vec<Vec<Note>>
{
    let current = cantus[step];
    let current_name = current.to_number();
    let current_octave = current.octave;

    if step == 0
    {
        let next = Note::from_number(current_name, current_octave + 1.0);

        intervals.push(0.0);
        counter.push(next);

        let mut valid_counters = Vec::<Vec::<Note>>::new();
        
        for mut c in counterpoint_first_species(cantus, counter, intervals, step + 1)
        {
            c.insert(0, next);

            valid_counters.push(c);
        }

        return valid_counters;
    }

    let last = counter[step - 1];

    if step == cantus.len() - 1
    {
        if matches!(last.name, Name::D) && matches!(cantus[step - 1].name, Name::B)
        {
            let next = Note::from_number_full(last.to_number_full() - 1.0);

            intervals.push(0.0);
            counter.push(next);

            return vec![vec![next]];
        }
        
        if matches!(last.name, Name::B) && matches!(cantus[step - 1].name, Name::D)
        {
            let next = Note::from_number_full(last.to_number_full() + 1.0);

            intervals.push(0.0);
            counter.push(next);

            return vec![vec![next]];
        }

        return vec![vec![]];
    }

    let mut valid_counters = Vec::<Vec<Note>>::new();

    for i in [1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 4.0, -4.0, 5.0, -5.0, 6.0, -6.0, 7.0, -7.0]
    {
        let next = Note::from_number_full(last.to_number_full() + i);

        if validate_first_species(cantus, counter, intervals, next, step)
        {
            let temp_intervals = &mut intervals.clone();
            let temp_counter = &mut counter.clone();

            temp_intervals.push((next - current) % 7.0 + 1.0);
            temp_counter.push(next);

            for mut c in counterpoint_first_species(cantus, temp_counter, temp_intervals, step + 1)
            {
                if c.len() > 0
                {
                    c.insert(0, next);

                    valid_counters.push(c);
                }
            }
        }
    }

    valid_counters
}

fn main()
{
    print!("Enter cantus firmus: ");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let cantus = input.trim_end().split(',').map(|x| Note::from_string(x)).collect::<Vec<Note>>();

    let options: Vec<Vec<Note>>;
    let best: Vec<&Vec<Note>>;

    options = counterpoint_first_species(&cantus, &mut Vec::new(), &mut Vec::new(), 0);

    let skips = options.iter().map(|x| x.clone().iter().skip(1).enumerate().map(|(i, y)| (*y - x[i]).abs()).sum::<f32>()).collect::<Vec<f32>>();
    let intervals = options.iter().map(|x| x.iter().enumerate().map(|(i, y)| *y - cantus[i]).collect()).collect::<Vec<Vec<f32>>>();

    let intervals = intervals.iter().map(|x|
    {
        let mut hash = HashMap::<usize, usize>::new();

        x.iter().for_each(|y| *hash.entry(*y as usize).or_insert(0) += 1);

        hash.values().product::<usize>()
    });

    let combined = skips.iter().zip(intervals).map(|(x, y)| *x as usize + y).collect::<Vec<usize>>();
    let min = *combined.iter().min().unwrap();

    best = options.iter().zip(combined).filter_map(|(x, y)| { if y == min { return Some(x); } None }).collect::<Vec<&Vec<Note>>>();

    let counter = best[0];

    let header = hound::WavSpec
    {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };

    print!("Enter output file location + name: ");

    input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut writer = hound::WavWriter::create(input.trim_end(), header).unwrap();

    for i in 0..cantus.len()
    {
        let cf_frequency = cantus[i].frequency();
        let cp_frequency = counter[i].frequency();

        let cf = (0..44100).map(|frame| get_sample(frame as f32, cf_frequency));
        let cp = (0..44100).map(|frame| get_sample(frame as f32, cp_frequency));

        for x in cf.zip(cp)
        {
            writer.write_sample(x.0).unwrap();
            writer.write_sample(x.1).unwrap();
        }
    }

    writer.finalize().unwrap();
}