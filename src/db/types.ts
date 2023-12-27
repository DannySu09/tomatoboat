export interface Topic {
  id: string;
  title: string;
  desc?: string;
  created_at: number;
  modified_at?: number;
  archived?: boolean;
}

export type NewTopic = Omit<Topic, 'id' | 'created_at' | 'modified_at'>;

export interface Work {
  id: number;
  topic_id: string;
  began_at: number;
  ended_at?: number;
  desc?: string;
}

export type NewWork = Omit<Work, 'id'>
