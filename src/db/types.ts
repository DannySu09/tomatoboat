export interface Topic {
  id: number;
  title: string;
  desc?: string;
  created_at: number;
  modified_at?: number;
}

export type NewTopic = Omit<Topic, 'id' | 'created_at' | 'modified_at'>;

export interface Event {
  id: number;
  title?: string;
  began_at: number;
  ended_at?: number;
}

export type NewEvent = Omit<Event, 'id'>;

export interface Work {
  id: number;
  event_id: number;
  began_at: number;
  ended_at?: number;
  desc?: string;
}

export type NewWork = Omit<Work, 'id'>
