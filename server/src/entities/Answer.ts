import { BaseEntity, Column, Entity, PrimaryGeneratedColumn } from "typeorm";
import { Question } from "./Question";

@Entity()
export class Answer extends BaseEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  question: Question;

  @Column("text")
  markdown: string;

  @Column()
  creatorId: number;

  @Column()
  accepted: boolean;
}
