import {
  BaseEntity,
  Column,
  Entity,
  OneToMany,
  PrimaryGeneratedColumn,
} from "typeorm";
import { Question } from "./Question";
import { Like } from "./Like";

@Entity()
export class User extends BaseEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column("text")
  name: string;

  @Column("text")
  profile_picture: string;

  @Column("text", { unique: true })
  githubId: string;

  @OneToMany(() => Question, (q) => q.creator)
  questions: Promise<Question[]>;

  @OneToMany(() => Like, (l) => l.user)
  likes: Promise<Like>;
}
