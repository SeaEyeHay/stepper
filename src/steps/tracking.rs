// Used by the datastructure
use std::collections::BinaryHeap;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering, Reverse};

// Traits
use std::iter::IntoIterator;


/**
 * Datas for actors curently performing a step
 */
#[derive(Debug)]
pub struct ActorsData<T> {
    pub ticks: u32,
    pub datas: Vec<T>,
}


impl<T> PartialEq for ActorsData<T> {
    fn eq(&self, other: &Self) -> bool { self.ticks == other.ticks }
}

impl<T> Eq for ActorsData<T> {}


impl<T> PartialOrd for ActorsData<T> {
     fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.ticks.partial_cmp(&other.ticks) } 
}

impl<T> Ord for ActorsData<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.ticks.cmp(&other.ticks) }
}

/***********************************************************************/

#[derive(Debug)]
pub struct ActorsTracker<T> {
    actors: BinaryHeap<Reverse<ActorsData<T>>>,
    current_tick: u32
}

impl<T> ActorsTracker<T> {
    /**
     * Constructor
     */
    pub fn new() -> Self {
        let new_heap = BinaryHeap::new();
        Self { actors: new_heap, current_tick: 0 }
    }


    /**
     * Methodes
     */
    pub fn insert(&mut self, tick: u32, datas: Vec<T>) {
        let tick = tick + self.current_tick;
        let new_entry = Reverse( ActorsData { ticks: tick, datas} );
        
        self.actors.push(new_entry);
    }

    pub fn remove(&mut self, tick: u32) {
        let tick = tick + self.current_tick;

        while self.actors.peek().filter(|Reverse(d)| d.ticks <= tick).is_some() {
            self.actors.pop();
        }
    }


    pub fn cleanup(self) -> Self {
        let mut new_tracker = Self::new();
        let mut group       = Vec::new();
        let mut tick        = self.current_tick;

        for Reverse(mut data) in self.actors.into_iter_sorted() {
            if data.ticks > tick {
                new_tracker.insert(tick - self.current_tick, group);

                group   = Vec::new();
                tick    = data.ticks;
            }

            group.append(&mut data.datas);
        }

        new_tracker
    }

    pub fn iter<'lt>(&'lt self) -> impl Iterator<Item = <&'lt Self as IntoIterator>::Item>
    where T: 'lt
    {
        self.into_iter()
    }
}

impl<'lt, T> IntoIterator for &'lt ActorsTracker<T> 
where T: 'lt
{
    type Item       = &'lt T;
    type IntoIter   = impl Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        let new_iter = self.actors.iter()
            .map(|Reverse(a)| a.datas.iter())
            .flatten();

        new_iter
    }
}


