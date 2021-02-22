import * as React from "react";
import {
  ChakraProvider,
  Box,
  Link,
  VStack,
  Grid,
  theme,
  Heading,
} from "@chakra-ui/react";
import { ColorModeSwitcher } from "../ColorModeSwitcher";
import Questions from "./Questions";

export default function Home() {
  const [show, setShow] = React.useState(false);

  if (show === false) {
    return (
      <ChakraProvider theme={theme}>
        <Box textAlign="center" fontSize="xl">
          <Grid minH="100vh" p={3}>
            <ColorModeSwitcher justifySelf="flex-end" />
            <VStack spacing={8}>
              <Heading>VSReview</Heading>
              <Link
                color="teal.500"
                fontSize="2xl"
                onClick={() => setShow(true)}
              >
                Ask or Answer a Question!
              </Link>
            </VStack>
          </Grid>
        </Box>
      </ChakraProvider>
    );
  } else {
    return (
      <ChakraProvider theme={theme}>
        <Box textAlign="center" fontSize="xl">
          <Grid minH="100vh" p={3}>
            <ColorModeSwitcher justifySelf="flex-end" />
            <Questions />
          </Grid>
        </Box>
      </ChakraProvider>
    );
  }
}
