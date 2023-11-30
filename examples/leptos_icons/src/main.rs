use leptos::*;
use leptos_icon_gen::leptos_icons;

leptos_icons! {
	pub FileExcelFilledIcon(ant_design_icons => FileExcelFilled),
	pub FileExcelOutlinedIcon(ant_design_icons => FileExcelOutlined),
	pub ArrowThroughHeartIcon(bootstrap_icons => ArrowThroughHeart),
	pub BellMinusIcon(boxicons => BxsBellMinusSolid),
	pub AlarmExclamationIcon(boxicons => BxsAlarmExclamationSolid),
	pub VuejsIcon(boxicons => BxlVuejsLogo),
	pub AppsMinusIcon(charm_icons => AppsMinus),
	pub ActivateBreakpointsCod(codicons => ActivateBreakpoints),
	pub AppleWatchIcon(css_gg => AppleWatch),
	pub RustIconFa(font_awesome => RustBrand),
	pub FileZipperFa(font_awesome => FileZipperRegular),
	pub ChatBubbleIcon(heroicons => ChatBubbleLeftRightOutline),
	pub ChatBubbleMiniIcon(heroicons => ChatBubbleLeftRightMini),
	pub ArchiveBoxIcon(heroicons => ArchiveBoxArrowDownSolid),
	pub PizzaIonIcon(ionicons => Pizza),
	pub PizzaOutlineIonIcon(ionicons => PizzaOutline),
	pub PizzaSharpIonIcon(ionicons => PizzaSharp),
	pub HomeIcon(lucide => Home),
	pub BoxTickOMicrons(microns => BoxTickO),
	pub AlertFill12Oct(octicons => AlertFill12),
	pub LogoGithub16Oct(octicons => LogoGithub16),
	pub Flame24Oct(octicons => Flame24),
	pub CapsuleFillRemix(remixicon => CapsuleFillHealthMedical),
	pub ChatSmileFillRemix(remixicon => ChatSmile2FillCommunication),
	pub AdventOfCodeSimple(simple_icons => Adventofcode),
	pub AbacusTabler(tabler_icons => Abacus),
}

fn main() {
	mount_to_body(|| {
		let size = Signal::derive(move || "64".to_string());

		view! {
			<div>
				<FileExcelFilledIcon width=size height=size/>
				<FileExcelOutlinedIcon width=size height=size stroke_width="2px"/>
				<ArrowThroughHeartIcon width=size height=size/>
				<BellMinusIcon width=size height=size/>
				<AlarmExclamationIcon width=size height=size/>
				<VuejsIcon width=size height=size/>
				<AppsMinusIcon width=size height=size/>
				<ActivateBreakpointsCod width=size height=size/>
				<AppleWatchIcon width=size height=size/>
				<RustIconFa width=size height=size/>
				<FileZipperFa width=size height=size/>
				<ChatBubbleIcon width=size height=size/>
				<ChatBubbleMiniIcon width=size height=size/>
				<ArchiveBoxIcon width=size height=size/>
				<PizzaIonIcon width=size height=size/>
				<PizzaOutlineIonIcon width=size height=size/>
				<PizzaSharpIonIcon width=size height=size/>
				<BoxTickOMicrons width=size height=size/>
				<AlertFill12Oct width=size height=size/>
				<LogoGithub16Oct width=size height=size/>
				<Flame24Oct width=size height=size/>
				<HomeIcon width=size height=size/>
				<CapsuleFillRemix width=size height=size/>
				<ChatSmileFillRemix width=size height=size/>
				<AdventOfCodeSimple width=size height=size/>
				<AbacusTabler width=size height=size/>
			</div>
		}
	})
}
