// This file was automatically generated by ts_bind, do not modify it manually

export interface Move {
   id: number | null;
   game_id: number;
   move_number: number;
   move_san: string;
   annotation: string | null;
   variation_order: number | null;
   parent_position_id: number;
   child_position_id: number;
}