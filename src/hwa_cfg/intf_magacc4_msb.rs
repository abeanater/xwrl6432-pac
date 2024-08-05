#[doc = "Register `INTF_MAGACC4_MSB` reader"]
pub type R = crate::R<IntfMagacc4MsbSpec>;
#[doc = "Register `INTF_MAGACC4_MSB` writer"]
pub type W = crate::W<IntfMagacc4MsbSpec>;
#[doc = "Field `INTF_MAGACC4_MSB` reader - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 3"]
pub type IntfMagacc4MsbR = crate::FieldReader;
#[doc = "Field `INTF_MAGACC4_MSB` writer - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 3"]
pub type IntfMagacc4MsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 3"]
    #[inline(always)]
    pub fn intf_magacc4_msb(&self) -> IntfMagacc4MsbR {
        IntfMagacc4MsbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register contains the accumulator value of the interference magnitude (MSB 4 bits) for bcnt = 3"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magacc4_msb(&mut self) -> IntfMagacc4MsbW<IntfMagacc4MsbSpec> {
        IntfMagacc4MsbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagacc4MsbSpec> {
        Nu1W::new(self, 4)
    }
}
#[doc = "INTF_MAGACC4_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc4_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc4_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc4MsbSpec;
impl crate::RegisterSpec for IntfMagacc4MsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc4_msb::R`](R) reader structure"]
impl crate::Readable for IntfMagacc4MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc4_msb::W`](W) writer structure"]
impl crate::Writable for IntfMagacc4MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC4_MSB to value 0"]
impl crate::Resettable for IntfMagacc4MsbSpec {
    const RESET_VALUE: u32 = 0;
}