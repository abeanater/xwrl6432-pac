#[doc = "Register `CPUCLKCTL` reader"]
pub type R = crate::R<CpuclkctlSpec>;
#[doc = "Register `CPUCLKCTL` writer"]
pub type W = crate::W<CpuclkctlSpec>;
#[doc = "Field `srcsel` reader - 3:0\\]
Slect the source clock:#br#0x0 : XTALCLK#br#0x1 : XTALCLKX2#br#0x2 : MDLL#br#0x3 : APLL/DPLL#br#0x4 : RCCLK#br#For other values if the lower 3 bits matches with above, corresponding clock is selected."]
pub type SrcselR = crate::FieldReader;
#[doc = "Field `srcsel` writer - 3:0\\]
Slect the source clock:#br#0x0 : XTALCLK#br#0x1 : XTALCLKX2#br#0x2 : MDLL#br#0x3 : APLL/DPLL#br#0x4 : RCCLK#br#For other values if the lower 3 bits matches with above, corresponding clock is selected."]
pub type SrcselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gate` reader - 7:4\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type GateR = crate::FieldReader;
#[doc = "Field `gate` writer - 7:4\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type GateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `divr` reader - 11:8\\]
Divide value#br#0x0 : div1#br#0x1 : div2#br#0x2 : div3#br#.#br#.#br#0xF = div16"]
pub type DivrR = crate::FieldReader;
#[doc = "Field `divr` writer - 11:8\\]
Divide value#br#0x0 : div1#br#0x1 : div2#br#0x2 : div3#br#.#br#.#br#0xF = div16"]
pub type DivrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `switchen` reader - 12:12\\]
1'b1 : Switch to the new divide ratio set by divr bits above. This happens when output clock of the divider is transitioning from 0->1,"]
pub type SwitchenR = crate::BitReader;
#[doc = "Field `switchen` writer - 12:12\\]
1'b1 : Switch to the new divide ratio set by divr bits above. This happens when output clock of the divider is transitioning from 0->1,"]
pub type SwitchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `currdivr` reader - 19:16\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrR = crate::FieldReader;
#[doc = "Field `currdivr` writer - 19:16\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `currclk` reader - 31:24\\]
Current Clock selected by GCM Clock Mux#br#0x1 : XTALCLK#br#0x2 : XTALCLKX2#br#0x4 : MDLL#br#0x8 : APLL/DPLL#br#0x10 : RCCLK"]
pub type CurrclkR = crate::FieldReader;
#[doc = "Field `currclk` writer - 31:24\\]
Current Clock selected by GCM Clock Mux#br#0x1 : XTALCLK#br#0x2 : XTALCLKX2#br#0x4 : MDLL#br#0x8 : APLL/DPLL#br#0x10 : RCCLK"]
pub type CurrclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Slect the source clock:#br#0x0 : XTALCLK#br#0x1 : XTALCLKX2#br#0x2 : MDLL#br#0x3 : APLL/DPLL#br#0x4 : RCCLK#br#For other values if the lower 3 bits matches with above, corresponding clock is selected."]
    #[inline(always)]
    pub fn srcsel(&self) -> SrcselR {
        SrcselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn gate(&self) -> GateR {
        GateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Divide value#br#0x0 : div1#br#0x1 : div2#br#0x2 : div3#br#.#br#.#br#0xF = div16"]
    #[inline(always)]
    pub fn divr(&self) -> DivrR {
        DivrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
1'b1 : Switch to the new divide ratio set by divr bits above. This happens when output clock of the divider is transitioning from 0->1,"]
    #[inline(always)]
    pub fn switchen(&self) -> SwitchenR {
        SwitchenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    pub fn currdivr(&self) -> CurrdivrR {
        CurrdivrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Current Clock selected by GCM Clock Mux#br#0x1 : XTALCLK#br#0x2 : XTALCLKX2#br#0x4 : MDLL#br#0x8 : APLL/DPLL#br#0x10 : RCCLK"]
    #[inline(always)]
    pub fn currclk(&self) -> CurrclkR {
        CurrclkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Slect the source clock:#br#0x0 : XTALCLK#br#0x1 : XTALCLKX2#br#0x2 : MDLL#br#0x3 : APLL/DPLL#br#0x4 : RCCLK#br#For other values if the lower 3 bits matches with above, corresponding clock is selected."]
    #[inline(always)]
    #[must_use]
    pub fn srcsel(&mut self) -> SrcselW<CpuclkctlSpec> {
        SrcselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn gate(&mut self) -> GateW<CpuclkctlSpec> {
        GateW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Divide value#br#0x0 : div1#br#0x1 : div2#br#0x2 : div3#br#.#br#.#br#0xF = div16"]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DivrW<CpuclkctlSpec> {
        DivrW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
1'b1 : Switch to the new divide ratio set by divr bits above. This happens when output clock of the divider is transitioning from 0->1,"]
    #[inline(always)]
    #[must_use]
    pub fn switchen(&mut self) -> SwitchenW<CpuclkctlSpec> {
        SwitchenW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn currdivr(&mut self) -> CurrdivrW<CpuclkctlSpec> {
        CurrdivrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Current Clock selected by GCM Clock Mux#br#0x1 : XTALCLK#br#0x2 : XTALCLKX2#br#0x4 : MDLL#br#0x8 : APLL/DPLL#br#0x10 : RCCLK"]
    #[inline(always)]
    #[must_use]
    pub fn currclk(&mut self) -> CurrclkW<CpuclkctlSpec> {
        CurrclkW::new(self, 24)
    }
}
#[doc = "CPUCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuclkctlSpec;
impl crate::RegisterSpec for CpuclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuclkctl::R`](R) reader structure"]
impl crate::Readable for CpuclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuclkctl::W`](W) writer structure"]
impl crate::Writable for CpuclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCLKCTL to value 0"]
impl crate::Resettable for CpuclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
