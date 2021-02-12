import {
  BaseEntity,
  Column,
  Entity,
  JoinColumn,
  ManyToOne,
  PrimaryGeneratedColumn,
} from "typeorm";
import { User } from "./User";
import { Answer } from "./Answer";
import { Like } from "./Like";

@Entity()
export class Question extends BaseEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column("text")
  title: string;

  @Column("text")
  markdown: string;

  @Column()
  creatorId: number;

  @ManyToOne(() => User, (u) => u.questions)
  @JoinColumn({ name: "creatorId" })
  creator: Promise<User>;

  @ManyToOne(() => Answer, (a) => a.markdown)
  @JoinColumn({ name: "question" })
  answers: Promise<Answer[]>;

  @ManyToOne(() => Like, (l) => l.question)
  @JoinColumn({ name: "question" })
  likes: Promise<Like[]>;
}
