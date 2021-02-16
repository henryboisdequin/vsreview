import {
  BaseEntity,
  Column,
  Entity,
  PrimaryGeneratedColumn,
  ManyToOne,
  JoinColumn,
} from "typeorm";
import { Question } from "./Question";
import { User } from "./User";

@Entity()
export class Answer extends BaseEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column(() => Question)
  question: Question;

  @Column("text")
  markdown: string;

  @Column()
  creatorId: number;

  @ManyToOne(() => User, (u) => u.answers)
  @JoinColumn({ name: "creatorId" })
  creator: Promise<User>;

  @Column()
  accepted: boolean;
}
