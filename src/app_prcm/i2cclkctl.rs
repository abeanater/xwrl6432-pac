#[doc = "Register `I2CCLKCTL` reader"]
pub type R = crate::R<I2cclkctlSpec>;
#[doc = "Register `I2CCLKCTL` writer"]
pub type W = crate::W<I2cclkctlSpec>;
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
impl R {
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
}
impl W {
    #[doc = "Bits 4:7 - 7:4\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn gate(&mut self) -> GateW<I2cclkctlSpec> {
        GateW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Divide value#br#0x0 : div1#br#0x1 : div2#br#0x2 : div3#br#.#br#.#br#0xF = div16"]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DivrW<I2cclkctlSpec> {
        DivrW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
1'b1 : Switch to the new divide ratio set by divr bits above. This happens when output clock of the divider is transitioning from 0->1,"]
    #[inline(always)]
    #[must_use]
    pub fn switchen(&mut self) -> SwitchenW<I2cclkctlSpec> {
        SwitchenW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn currdivr(&mut self) -> CurrdivrW<I2cclkctlSpec> {
        CurrdivrW::new(self, 16)
    }
}
#[doc = "I2CCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cclkctlSpec;
impl crate::RegisterSpec for I2cclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cclkctl::R`](R) reader structure"]
impl crate::Readable for I2cclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cclkctl::W`](W) writer structure"]
impl crate::Writable for I2cclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2CCLKCTL to value 0"]
impl crate::Resettable for I2cclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
