import { Heading, Text, Box, Button } from "@chakra-ui/react";
import React from "react";
import ReactMarkdown from "react-markdown";

interface Question {
  title: String;
  text: String;
  answers: Array<Answer>;
}

interface Answer {
  text: String;
}

export default function Questions() {
  const [questions, setQuestions] = React.useState<Array<Question>>([]);

  const createQuestion = () => {
    const title: String =
      window.prompt("What is the title of your question?") ||
      "Untitled Question";
    const text: String =
      window.prompt("What is your question?") || "Empty Text";
    setQuestions([{ title, text, answers: [] }, ...questions]);
  };

  const createAnswer = (q: Question, idx: number) => {
    questions.splice(idx, 1);
    const text: String =
      window.prompt("What is the answer to this question?") || "Empty Text";
    q.answers.push({ text });
    setQuestions([q, ...questions]);
  };

  return (
    <>
      <Box>
        <Heading marginBottom="4">Questions</Heading>
        <Button
          marginBottom="4"
          maxWidth="200px"
          onClick={() => createQuestion()}
        >
          Ask a Question
        </Button>
      </Box>
      {questions.map((q, idx) => {
        return (
          <Box border="1px solid white">
            <Heading marginBottom="4">{q.title}</Heading>
            <Text>
              Question: <ReactMarkdown source={q.text as string} />
            </Text>
            <Heading marginBottom="4" marginTop="4">
              {q.answers.length > 0 ? "Answers:" : ""}
            </Heading>
            <Button
              marginBottom="4"
              maxWidth="200px"
              onClick={() => createAnswer(q, idx)}
            >
              Answer this Question
            </Button>
            {q.answers.map((a) => {
              return (
                <Box>
                  <Text>
                    Answer: <ReactMarkdown source={a.text as string} />
                  </Text>
                </Box>
              );
            })}
          </Box>
        );
      })}
    </>
  );
}
