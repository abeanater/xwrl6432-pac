#[doc = "Register `INTF_MAG_SCALE` reader"]
pub type R = crate::R<IntfMagScaleSpec>;
#[doc = "Register `INTF_MAG_SCALE` writer"]
pub type W = crate::W<IntfMagScaleSpec>;
#[doc = "Field `INTF_MAG_SCALE` reader - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block.Default 8= scale of 1.0"]
pub type IntfMagScaleR = crate::FieldReader;
#[doc = "Field `INTF_MAG_SCALE` writer - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block.Default 8= scale of 1.0"]
pub type IntfMagScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block.Default 8= scale of 1.0"]
    #[inline(always)]
    pub fn intf_mag_scale(&self) -> IntfMagScaleR {
        IntfMagScaleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGn from interference statistics block.Default 8= scale of 1.0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_mag_scale(&mut self) -> IntfMagScaleW<IntfMagScaleSpec> {
        IntfMagScaleW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagScaleSpec> {
        Nu1W::new(self, 8)
    }
}
#[doc = "INTF_MAG_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mag_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mag_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagScaleSpec;
impl crate::RegisterSpec for IntfMagScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_mag_scale::R`](R) reader structure"]
impl crate::Readable for IntfMagScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_mag_scale::W`](W) writer structure"]
impl crate::Writable for IntfMagScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAG_SCALE to value 0"]
impl crate::Resettable for IntfMagScaleSpec {
    const RESET_VALUE: u32 = 0;
}
