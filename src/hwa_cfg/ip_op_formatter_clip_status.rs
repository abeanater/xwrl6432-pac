#[doc = "Register `IP_OP_FORMATTER_CLIP_STATUS` reader"]
pub type R = crate::R<IpOpFormatterClipStatusSpec>;
#[doc = "Register `IP_OP_FORMATTER_CLIP_STATUS` writer"]
pub type W = crate::W<IpOpFormatterClipStatusSpec>;
#[doc = "Field `IP_FORMATTER_CLIP_STATUS` reader - 0:0\\]
Indicates input formatter clip status"]
pub type IpFormatterClipStatusR = crate::BitReader;
#[doc = "Field `IP_FORMATTER_CLIP_STATUS` writer - 0:0\\]
Indicates input formatter clip status"]
pub type IpFormatterClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `OP_FORMATTER_CLIP_STATUS` reader - 16:16\\]
Indicates output formatter clip status"]
pub type OpFormatterClipStatusR = crate::BitReader;
#[doc = "Field `OP_FORMATTER_CLIP_STATUS` writer - 16:16\\]
Indicates output formatter clip status"]
pub type OpFormatterClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates input formatter clip status"]
    #[inline(always)]
    pub fn ip_formatter_clip_status(&self) -> IpFormatterClipStatusR {
        IpFormatterClipStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates output formatter clip status"]
    #[inline(always)]
    pub fn op_formatter_clip_status(&self) -> OpFormatterClipStatusR {
        OpFormatterClipStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates input formatter clip status"]
    #[inline(always)]
    #[must_use]
    pub fn ip_formatter_clip_status(
        &mut self,
    ) -> IpFormatterClipStatusW<IpOpFormatterClipStatusSpec> {
        IpFormatterClipStatusW::new(self, 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IpOpFormatterClipStatusSpec> {
        Nu1W::new(self, 1)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates output formatter clip status"]
    #[inline(always)]
    #[must_use]
    pub fn op_formatter_clip_status(
        &mut self,
    ) -> OpFormatterClipStatusW<IpOpFormatterClipStatusSpec> {
        OpFormatterClipStatusW::new(self, 16)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<IpOpFormatterClipStatusSpec> {
        Nu2W::new(self, 17)
    }
}
#[doc = "IP_OP_FORMATTER_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_op_formatter_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip_op_formatter_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpOpFormatterClipStatusSpec;
impl crate::RegisterSpec for IpOpFormatterClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_op_formatter_clip_status::R`](R) reader structure"]
impl crate::Readable for IpOpFormatterClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ip_op_formatter_clip_status::W`](W) writer structure"]
impl crate::Writable for IpOpFormatterClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IP_OP_FORMATTER_CLIP_STATUS to value 0"]
impl crate::Resettable for IpOpFormatterClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
