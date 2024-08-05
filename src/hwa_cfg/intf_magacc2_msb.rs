#[doc = "Register `INTF_MAGACC2_MSB` reader"]
pub type R = crate::R<IntfMagacc2MsbSpec>;
#[doc = "Register `INTF_MAGACC2_MSB` writer"]
pub type W = crate::W<IntfMagacc2MsbSpec>;
#[doc = "Field `INTF_MAGACC2_MSB` reader - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 1"]
pub type IntfMagacc2MsbR = crate::FieldReader;
#[doc = "Field `INTF_MAGACC2_MSB` writer - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 1"]
pub type IntfMagacc2MsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 1"]
    #[inline(always)]
    pub fn intf_magacc2_msb(&self) -> IntfMagacc2MsbR {
        IntfMagacc2MsbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 1"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magacc2_msb(&mut self) -> IntfMagacc2MsbW<IntfMagacc2MsbSpec> {
        IntfMagacc2MsbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagacc2MsbSpec> {
        Nu1W::new(self, 4)
    }
}
#[doc = "INTF_MAGACC2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc2_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc2_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc2MsbSpec;
impl crate::RegisterSpec for IntfMagacc2MsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc2_msb::R`](R) reader structure"]
impl crate::Readable for IntfMagacc2MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc2_msb::W`](W) writer structure"]
impl crate::Writable for IntfMagacc2MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC2_MSB to value 0"]
impl crate::Resettable for IntfMagacc2MsbSpec {
    const RESET_VALUE: u32 = 0;
}
