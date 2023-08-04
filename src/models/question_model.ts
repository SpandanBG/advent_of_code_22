enum DescriptionType {
    Paragraph = "p",
    Code = "code",
    List = "li",
}

interface DesciptionI {
    type: DescriptionType,
    values: string[],
}

interface QuestionI {
    id: string,
    title: string,
    part_1: DesciptionI[],
    part_2: DesciptionI[],
}

export { DescriptionType };
export type { QuestionI };