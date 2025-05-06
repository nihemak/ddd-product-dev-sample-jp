import type { Meta, StoryObj } from "@storybook/react";

import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from "./select";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
	title: "UI/Select",
	component: Select,
	parameters: {
		// Optional parameter to center the component in the Canvas. More info: https://storybook.js.org/docs/configure/story-layout
		layout: "centered",
	},
	// This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
	tags: ["autodocs"],
	args: {
		// Define default args here if needed, although Select might not need many direct args
	},
	argTypes: {
		// Define argTypes for controls if needed
	},
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args

const renderSelect = (args: Story['args']) => (
	<Select {...args}>
		<SelectTrigger className="w-[280px]">
			<SelectValue placeholder="届け先を選択してください" />
		</SelectTrigger>
		<SelectContent>
			<SelectGroup>
				<SelectLabel>登録済みの届け先</SelectLabel>
				<SelectItem value="address-1">自宅 (山田 太郎)</SelectItem>
				<SelectItem value="address-2">勤務先 (田中 花子)</SelectItem>
				<SelectItem value="address-3">友人宅 (鈴木 一郎)</SelectItem>
			</SelectGroup>
		</SelectContent>
	</Select>
);

export const Default: Story = {
	render: renderSelect,
	args: {
		// Args for the default unselected state
	},
};

export const Selected: Story = {
	render: renderSelect,
	args: {
		defaultValue: "address-2", // Example: Pre-select the second option
	},
}; 